// swift-tools-version:6.1
// The swift-tools-version declares the minimum version of Swift required to build this package.
import PackageDescription

let package = Package(
	name: "DepsOnly",
	dependencies: [
		.package(url: "https://github.com/Governikus/AusweisApp2-SDK-iOS",
			exact: "2.5.4"),
		.package(url: "https://github.com/Governikus/AusweisApp2Wrapper-iOS-SPM",
			exact: "2.5.4"),
		.package(url: "https://github.com/pointfreeco/combine-schedulers",
			exact: "1.2.0"),
		.package(url: "https://github.com/devicekit/DeviceKit",
			exact: "5.8.0"),
		.package(url: "https://github.com/SFSafeSymbols/SFSafeSymbols",
			exact: "7.0.0"),
		.package(url: "https://github.com/pointfreeco/swift-case-paths",
			exact: "1.9.1"),
		.package(url: "https://github.com/pointfreeco/swift-clocks",
			exact: "1.1.0"),
		.package(url: "https://github.com/apple/swift-collections",
			exact: "1.6.0"),
		.package(url: "https://github.com/pointfreeco/swift-composable-architecture",
			exact: "1.26.1"),
		.package(url: "https://github.com/pointfreeco/swift-concurrency-extras",
			exact: "1.4.1"),
		.package(url: "https://github.com/pointfreeco/swift-custom-dump",
			exact: "1.7.0"),
		.package(url: "https://github.com/pointfreeco/swift-dependencies",
			exact: "1.17.0"),
		.package(url: "https://github.com/pointfreeco/swift-identified-collections",
			exact: "1.1.1"),
		.package(url: "https://github.com/pointfreeco/swift-navigation",
			exact: "2.10.3"),
		.package(url: "https://github.com/pointfreeco/swift-perception",
			exact: "2.0.11"),
		.package(url: "https://github.com/pointfreeco/swift-sharing",
			exact: "2.9.1"),
		.package(url: "https://github.com/swiftlang/swift-syntax",
			exact: "602.0.0"),
		.package(url: "https://github.com/pointfreeco/xctest-dynamic-overlay",
			exact: "1.11.0")
	],
	targets: [
		.target(name: "DepsOnly", dependencies: [
			"ausweisapp2-sdk-ios",
			"ausweisapp2wrapper-ios-spm",
			"combine-schedulers",
			"devicekit",
			"sfsafesymbols",
			"swift-case-paths",
			"swift-clocks",
			"swift-collections",
			"swift-composable-architecture",
			"swift-concurrency-extras",
			"swift-custom-dump",
			"swift-dependencies",
			"swift-identified-collections",
			"swift-navigation",
			"swift-perception",
			"swift-sharing",
			"swift-syntax",
			"xctest-dynamic-overlay"
		])
	]
)
