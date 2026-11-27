//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Combine
import SwiftUI

enum SheetDetent: Hashable {
    case height(CGFloat)
    case fraction(CGFloat)
    case large

    func resolve(containerHeight: CGFloat, topInset: CGFloat) -> CGFloat {
        let maxHeight = max(0, containerHeight - topInset - Self.topGap)
        switch self {
        case .height(let value):
            return min(value, maxHeight)
        case .fraction(let fraction):
            return min(containerHeight * fraction, maxHeight)
        case .large:
            return maxHeight
        }
    }

    static let topGap: CGFloat = 32
}

/// We need a custom sheet equivalent due to how the reply confirmation dialog
/// is designed. It should appear above the reply view sheet and that is not
/// possible with the standard `.sheet` modifier.
struct DraggableSheetOverlayView<Header: View, Content: View>: View {
    let detents: [SheetDetent]
    @Binding var selection: SheetDetent
    let interactiveThreshold: SheetDetent
    @ViewBuilder let header: () -> Header
    @ViewBuilder let content: () -> Content

    @State private var containerHeight = 0.0
    @State private var topInset = 0.0
    @State private var bottomInset = 0.0
    @State private var keyboardHeight = 0.0
    @State private var dragTranslation = 0.0

    private static var snapAnimation: Animation { .spring(response: 0.35, dampingFraction: 0.85) }
    private static var dimAnimation: Animation { .easeInOut(duration: 0.2) }

    private var keyboardPublisher: AnyPublisher<CGFloat, Never> {
        let willChange = NotificationCenter.default
            .publisher(for: UIResponder.keyboardWillChangeFrameNotification)
            .compactMap { notification -> CGFloat? in
                (notification.userInfo?[UIResponder.keyboardFrameEndUserInfoKey] as? CGRect)?.height
            }
        let willHide = NotificationCenter.default
            .publisher(for: UIResponder.keyboardWillHideNotification)
            .map { _ in CGFloat(0) }
        return willChange.merge(with: willHide).eraseToAnyPublisher()
    }

    private var keyboardOffset: CGFloat {
        max(0, keyboardHeight - bottomInset)
    }

    private func resolvedHeight(for detent: SheetDetent) -> CGFloat {
        detent.resolve(containerHeight: containerHeight, topInset: topInset)
    }

    private var minDetentHeight: CGFloat {
        detents.map { resolvedHeight(for: $0) }.min() ?? 0
    }

    private var maxDetentHeight: CGFloat {
        detents.map { resolvedHeight(for: $0) }.max() ?? 0
    }

    private var draggedHeight: CGFloat {
        let base = resolvedHeight(for: selection) - dragTranslation
        return min(max(base, minDetentHeight), maxDetentHeight)
    }

    private var availableHeight: CGFloat {
        max(0, containerHeight - topInset - SheetDetent.topGap - keyboardOffset)
    }

    private var sheetHeight: CGFloat {
        min(draggedHeight, availableHeight)
    }

    private var dimProgress: CGFloat {
        let threshold = resolvedHeight(for: interactiveThreshold)
        let ceiling = maxDetentHeight
        guard ceiling > threshold else { return 0 }
        return min(max((draggedHeight - threshold) / (ceiling - threshold), 0), 1)
    }

    private var isBlockingBackground: Bool {
        draggedHeight > resolvedHeight(for: interactiveThreshold) + 1
    }

    private func nearestDetent(predictedEndTranslation: CGFloat) -> SheetDetent {
        let projectedHeight = resolvedHeight(for: selection) - predictedEndTranslation
        return detents.min { lhs, rhs in
            abs(resolvedHeight(for: lhs) - projectedHeight) < abs(resolvedHeight(for: rhs) - projectedHeight)
        } ?? selection
    }

    private var dragGesture: some Gesture {
        DragGesture(minimumDistance: 10, coordinateSpace: .global)
            .onChanged { value in
                dragTranslation = value.translation.height
            }
            .onEnded { value in
                let predicted = value.predictedEndTranslation.height
                withAnimation(Self.snapAnimation) {
                    if abs(value.translation.height) > 12 {
                        selection = nearestDetent(predictedEndTranslation: predicted)
                    }
                    dragTranslation = 0
                }
            }
    }

    var body: some View {
        ZStack(alignment: .bottom) {
            Color.black
                .opacity(0.2 * dimProgress)
                .ignoresSafeArea()
                .allowsHitTesting(isBlockingBackground)
                .animation(Self.dimAnimation, value: isBlockingBackground)

            VStack(spacing: 0) {
                header()
                    .frame(maxWidth: .infinity)
                    .contentShape(Rectangle())
                    .simultaneousGesture(dragGesture)

                content()
                    .frame(maxWidth: .infinity)
            }
            .frame(maxWidth: .infinity, alignment: .top)
            .frame(height: sheetHeight, alignment: .top)
            .clipped()
            .background(
                UnevenRoundedRectangle(topLeadingRadius: 16, topTrailingRadius: 16)
                    .fill(Asset.Theme.inputBackground.swiftUIColor)
                    .shadow(color: .black.opacity(0.15), radius: 12, y: -2)
                    .ignoresSafeArea(edges: .bottom)
            )
            .padding(.bottom, keyboardOffset)
            .animation(Self.snapAnimation, value: keyboardOffset)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .bottom)
        .ignoresSafeArea(.keyboard)
        .onAppear { updateMetrics() }
        .onReceive(keyboardPublisher) { height in
            updateMetrics()
            withAnimation(Self.snapAnimation) {
                keyboardHeight = height
            }
        }
    }

    @MainActor
    private func updateMetrics() {
        guard let window = UIApplication.shared.connectedScenes
            .compactMap({ $0 as? UIWindowScene })
            .flatMap(\.windows)
            .first(where: { $0.isKeyWindow }) else { return }
        containerHeight = window.bounds.height
        topInset = window.safeAreaInsets.top
        bottomInset = window.safeAreaInsets.bottom
    }
}
