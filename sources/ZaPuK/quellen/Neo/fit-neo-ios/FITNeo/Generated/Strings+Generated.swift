// swiftlint:disable all
// Generated using SwiftGen — https://github.com/SwiftGen/SwiftGen

import Foundation

// swiftlint:disable superfluous_disable_command file_length implicit_return prefer_self_in_static_references

// MARK: - Strings

// swiftlint:disable explicit_type_interface function_parameter_count identifier_name line_length
// swiftlint:disable nesting type_body_length type_name vertical_whitespace_opening_braces
internal enum L10n {
  internal enum Alert {
    /// An error occurred
    internal static let errorOccurred = L10n.tr("Localizable", "alert.error_occurred", fallback: "An error occurred")
    /// OK
    internal static let ok = L10n.tr("Localizable", "alert.ok", fallback: "OK")
  }
  internal enum App {
    /// My Digital Administration
    internal static let displayName = L10n.tr("Localizable", "app.display_name", fallback: "My Digital Administration")
  }
  internal enum Navigation {
    internal enum Tab {
      /// Account
      internal static let account = L10n.tr("Localizable", "navigation.tab.account", fallback: "Account")
      /// Documents
      internal static let documents = L10n.tr("Localizable", "navigation.tab.documents", fallback: "Documents")
      /// Help
      internal static let help = L10n.tr("Localizable", "navigation.tab.help", fallback: "Help")
      /// Cases
      internal static let processes = L10n.tr("Localizable", "navigation.tab.processes", fallback: "Cases")
    }
  }
  internal enum Processes {
    /// Unknown Authority
    internal static let unknownAuthority = L10n.tr("Localizable", "processes.unknown_authority", fallback: "Unknown Authority")
  }
  internal enum Screen {
    /// Attachments
    internal static let attachments = L10n.tr("Localizable", "screen.attachments", fallback: "Attachments")
    /// Expand details
    internal static let expandDetails = L10n.tr("Localizable", "screen.expand_details", fallback: "Expand details")
    /// Hide details
    internal static let hideDetails = L10n.tr("Localizable", "screen.hide_details", fallback: "Hide details")
    /// Information
    internal static let information = L10n.tr("Localizable", "screen.information", fallback: "Information")
    /// Loading...
    internal static let loading = L10n.tr("Localizable", "screen.loading", fallback: "Loading...")
    /// Loading failed!
    internal static let loadingFailed = L10n.tr("Localizable", "screen.loading_failed", fallback: "Loading failed!")
    /// This is not implemented yet
    internal static let notImplemented = L10n.tr("Localizable", "screen.not_implemented", fallback: "This is not implemented yet")
    /// Show more information
    internal static let showMoreInfo = L10n.tr("Localizable", "screen.show_more_info", fallback: "Show more information")
    internal enum Action {
      /// Answer now
      internal static let replyNow = L10n.tr("Localizable", "screen.action.reply_now", fallback: "Answer now")
    }
    internal enum Help {
      /// Unable to open the web browser.
      internal static let browserNotSupported = L10n.tr("Localizable", "screen.help.browser_not_supported", fallback: "Unable to open the web browser.")
      /// Call 115
      internal static let call = L10n.tr("Localizable", "screen.help.call", fallback: "Call 115")
      /// Phone calls are not possible on this device.
      internal static let callsNotSupported = L10n.tr("Localizable", "screen.help.calls_not_supported", fallback: "Phone calls are not possible on this device.")
      /// Open Help
      internal static let openHelp = L10n.tr("Localizable", "screen.help.open_help", fallback: "Open Help")
      /// The team at the authorities' number 115 answers all inquiries about your cases from Monday to Friday between 8:00 and 18:00.
      internal static let paragraph1 = L10n.tr("Localizable", "screen.help.paragraph1", fallback: "The team at the authorities' number 115 answers all inquiries about your cases from Monday to Friday between 8:00 and 18:00.")
      /// Alternatively, you can also use our help section to search for an answer to your specific question.
      internal static let paragraph2 = L10n.tr("Localizable", "screen.help.paragraph2", fallback: "Alternatively, you can also use our help section to search for an answer to your specific question.")
      /// Do you need help?
      internal static let title = L10n.tr("Localizable", "screen.help.title", fallback: "Do you need help?")
    }
    internal enum LoginError {
      /// Unable to authenticate
      internal static let authenticationFailed = L10n.tr("Localizable", "screen.login_error.authentication_failed", fallback: "Unable to authenticate")
    }
    internal enum LoginView {
      /// Log in with BundID
      internal static let button = L10n.tr("Localizable", "screen.login_view.button", fallback: "Log in with BundID")
      /// Easy communication with the administration
      internal static let subtitle = L10n.tr("Localizable", "screen.login_view.subtitle", fallback: "Easy communication with the administration")
    }
    internal enum Message {
      internal enum Reply {
        /// Add to answer
        internal static let addReply = L10n.tr("Localizable", "screen.message.reply.add_reply", fallback: "Add to answer")
        /// Add answer template
        internal static let addTemplate = L10n.tr("Localizable", "screen.message.reply.add_template", fallback: "Add answer template")
        /// Answered on time
        internal static let answered = L10n.tr("Localizable", "screen.message.reply.answered", fallback: "Answered on time")
        /// Clear answer content
        internal static let clearReply = L10n.tr("Localizable", "screen.message.reply.clear_reply", fallback: "Clear answer content")
        /// Collapse answer editor
        internal static let collapse = L10n.tr("Localizable", "screen.message.reply.collapse", fallback: "Collapse answer editor")
        /// Expand answer editor
        internal static let expand = L10n.tr("Localizable", "screen.message.reply.expand", fallback: "Expand answer editor")
        /// Message has no plain text content
        internal static let noContent = L10n.tr("Localizable", "screen.message.reply.no_content", fallback: "Message has no plain text content")
        /// Enter your answer...
        internal static let placeholder = L10n.tr("Localizable", "screen.message.reply.placeholder", fallback: "Enter your answer...")
        /// Answer required
        internal static let `required` = L10n.tr("Localizable", "screen.message.reply.required", fallback: "Answer required")
        /// Dear Sir or Madam,
        /// 
        /// Sincerely,
        /// %@
        internal static func template(_ p1: Any) -> String {
          return L10n.tr("Localizable", "screen.message.reply.template", String(describing: p1), fallback: "Dear Sir or Madam,\n\nSincerely,\n%@")
        }
        /// Message content is not supported
        internal static let unsupportedContentType = L10n.tr("Localizable", "screen.message.reply.unsupported_content_type", fallback: "Message content is not supported")
        internal enum Confirm {
          /// Answer:
          internal static let answer = L10n.tr("Localizable", "screen.message.reply.confirm.answer", fallback: "Answer:")
          /// Your information is binding. Please confirm if you want to send the answer.
          internal static let details = L10n.tr("Localizable", "screen.message.reply.confirm.details", fallback: "Your information is binding. Please confirm if you want to send the answer.")
          /// Sending answer...
          internal static let sending = L10n.tr("Localizable", "screen.message.reply.confirm.sending", fallback: "Sending answer...")
          /// Send answer?
          internal static let title = L10n.tr("Localizable", "screen.message.reply.confirm.title", fallback: "Send answer?")
          internal enum CheckDetails {
            /// Check details
            internal static let button = L10n.tr("Localizable", "screen.message.reply.confirm.check_details.button", fallback: "Check details")
          }
        }
        internal enum Polls {
          /// Add a comment...
          internal static let addComment = L10n.tr("Localizable", "screen.message.reply.polls.add_comment", fallback: "Add a comment...")
          /// Comment:
          internal static let comment = L10n.tr("Localizable", "screen.message.reply.polls.comment", fallback: "Comment:")
          /// None of the above answers
          internal static let noneOfTheAbove = L10n.tr("Localizable", "screen.message.reply.polls.none_of_the_above", fallback: "None of the above answers")
        }
        internal enum Submit {
          /// Send answer
          internal static let button = L10n.tr("Localizable", "screen.message.reply.submit.button", fallback: "Send answer")
        }
        internal enum Success {
          /// Done
          internal static let button = L10n.tr("Localizable", "screen.message.reply.success.button", fallback: "Done")
          /// Your answer has been successfully sent.
          internal static let details = L10n.tr("Localizable", "screen.message.reply.success.details", fallback: "Your answer has been successfully sent.")
          /// Thank you!
          internal static let title = L10n.tr("Localizable", "screen.message.reply.success.title", fallback: "Thank you!")
        }
      }
    }
    internal enum MessageDetailsView {
      /// Inquiry regarding your case
      internal static let caseInquiry = L10n.tr("Localizable", "screen.message_details_view.case_inquiry", fallback: "Inquiry regarding your case")
      /// Collapse message header
      internal static let collapseHeader = L10n.tr("Localizable", "screen.message_details_view.collapse_header", fallback: "Collapse message header")
      /// Expand message header
      internal static let expandHeader = L10n.tr("Localizable", "screen.message_details_view.expand_header", fallback: "Expand message header")
      /// Message details
      internal static let title = L10n.tr("Localizable", "screen.message_details_view.title", fallback: "Message details")
    }
    internal enum Messages {
      /// Your answer
      internal static let yourReply = L10n.tr("Localizable", "screen.messages.your_reply", fallback: "Your answer")
    }
    internal enum MessagesOverviewView {
      /// No messages received yet.
      internal static let noMessages = L10n.tr("Localizable", "screen.messages_overview_view.no_messages", fallback: "No messages received yet.")
      /// Once you have received a message regarding your application, you can view it here.
      internal static let noMessagesDetail = L10n.tr("Localizable", "screen.messages_overview_view.no_messages_detail", fallback: "Once you have received a message regarding your application, you can view it here.")
      /// Case Messages
      internal static let title = L10n.tr("Localizable", "screen.messages_overview_view.title", fallback: "Case Messages")
    }
    internal enum ProcessDetailsView {
      /// Case details
      internal static let title = L10n.tr("Localizable", "screen.process_details_view.title", fallback: "Case details")
    }
    internal enum ProcessesView {
      /// Simple, fast and secure communication for all your active official matters.
      internal static let benefit1 = L10n.tr("Localizable", "screen.processes_view.benefit_1", fallback: "Simple, fast and secure communication for all your active official matters.")
      /// Simple upload and overview of all your documents.
      internal static let benefit2 = L10n.tr("Localizable", "screen.processes_view.benefit_2", fallback: "Simple upload and overview of all your documents.")
      /// A wide range of helpful information and support for your administrative cases.
      internal static let benefit3 = L10n.tr("Localizable", "screen.processes_view.benefit_3", fallback: "A wide range of helpful information and support for your administrative cases.")
      /// Your completed cases
      internal static let completedProcesses = L10n.tr("Localizable", "screen.processes_view.completed_processes", fallback: "Your completed cases")
      /// Access the Federal Portal
      internal static let federalPortal = L10n.tr("Localizable", "screen.processes_view.federal_portal", fallback: "Access the Federal Portal")
      /// Hide completed cases
      internal static let hideCompleted = L10n.tr("Localizable", "screen.processes_view.hide_completed", fallback: "Hide completed cases")
      /// Once you have an active case with a specialist authority, you can take advantage of the benefits listed below.
      internal static let paragraph1 = L10n.tr("Localizable", "screen.processes_view.paragraph_1", fallback: "Once you have an active case with a specialist authority, you can take advantage of the benefits listed below.")
      /// Starting a new case with the administrative services of the federal government, states, and municipalities can be done online.
      internal static let paragraph2 = L10n.tr("Localizable", "screen.processes_view.paragraph_2", fallback: "Starting a new case with the administrative services of the federal government, states, and municipalities can be done online.")
      /// Show completed cases
      internal static let showCompleted = L10n.tr("Localizable", "screen.processes_view.show_completed", fallback: "Show completed cases")
      /// No cases pending.
      internal static let subHeader = L10n.tr("Localizable", "screen.processes_view.sub_header", fallback: "No cases pending.")
    }
    internal enum ReportsView {
      /// Reports
      internal static let title = L10n.tr("Localizable", "screen.reports_view.title", fallback: "Reports")
      internal enum Empty {
        /// There are currently no updates from your transactions.
        internal static let detail = L10n.tr("Localizable", "screen.reports_view.empty.detail", fallback: "There are currently no updates from your transactions.")
        /// No reports available
        internal static let title = L10n.tr("Localizable", "screen.reports_view.empty.title", fallback: "No reports available")
      }
    }
    internal enum StatusInfoListView {
      /// Status Explanation
      internal static let navigationTitle = L10n.tr("Localizable", "screen.status_info_list_view.navigation_title", fallback: "Status Explanation")
      internal enum Info {
        /// If this step is displayed, your activity is required. More information can be found in the corresponding messages.
        /// 
        /// If you need to respond within a certain timeframe, this will be indicated by the app.
        internal static let processActivityRequired = L10n.tr("Localizable", "screen.status_info_list_view.info.process_activity_required", fallback: "If this step is displayed, your activity is required. More information can be found in the corresponding messages.\n\nIf you need to respond within a certain timeframe, this will be indicated by the app.")
        /// The case has been successfully completed.
        internal static let processCompleted = L10n.tr("Localizable", "screen.status_info_list_view.info.process_completed", fallback: "The case has been successfully completed.")
        /// Your case has been received by the authority. It has not yet been reviewed, but the review will start soon.
        /// 
        /// If a time estimate for the authority's response can be provided, it will be displayed in the app.
        internal static let processReceived = L10n.tr("Localizable", "screen.status_info_list_view.info.process_received", fallback: "Your case has been received by the authority. It has not yet been reviewed, but the review will start soon.\n\nIf a time estimate for the authority's response can be provided, it will be displayed in the app.")
        /// You have completed the case and subsequently submitted it digitally to the authority.
        /// 
        /// If a time estimate for the authority's response can be provided, it will be displayed in the app.
        internal static let processSent = L10n.tr("Localizable", "screen.status_info_list_view.info.process_sent", fallback: "You have completed the case and subsequently submitted it digitally to the authority.\n\nIf a time estimate for the authority's response can be provided, it will be displayed in the app.")
        /// Your case is being reviewed and processed by the authority. You will receive feedback on your case soon.
        /// 
        /// If a time estimate for the authority's response can be provided, it will be displayed in the app.
        internal static let processUnderReview = L10n.tr("Localizable", "screen.status_info_list_view.info.process_under_review", fallback: "Your case is being reviewed and processed by the authority. You will receive feedback on your case soon.\n\nIf a time estimate for the authority's response can be provided, it will be displayed in the app.")
      }
    }
    internal enum Warning {
      /// Answer by %@
      internal static func messageDueDate(_ p1: Any) -> String {
        return L10n.tr("Localizable", "screen.warning.message_due_date", String(describing: p1), fallback: "Answer by %@")
      }
      /// You must answer at least to one message by %@
      internal static func processDueDate(_ p1: Any) -> String {
        return L10n.tr("Localizable", "screen.warning.process_due_date", String(describing: p1), fallback: "You must answer at least to one message by %@")
      }
    }
  }
  internal enum Status {
    /// New Message
    internal static let newMessage = L10n.tr("Localizable", "status.new_message", fallback: "New Message")
    /// Activity Required
    internal static let processActivityRequired = L10n.tr("Localizable", "status.process_activity_required", fallback: "Activity Required")
    /// Case Completed
    internal static let processCompleted = L10n.tr("Localizable", "status.process_completed", fallback: "Case Completed")
    /// Case Received
    internal static let processReceived = L10n.tr("Localizable", "status.process_received", fallback: "Case Received")
    /// Case Sent
    internal static let processSent = L10n.tr("Localizable", "status.process_sent", fallback: "Case Sent")
    /// Case Under Review
    internal static let processUnderReview = L10n.tr("Localizable", "status.process_under_review", fallback: "Case Under Review")
  }
}
// swiftlint:enable explicit_type_interface function_parameter_count identifier_name line_length
// swiftlint:enable nesting type_body_length type_name vertical_whitespace_opening_braces

// MARK: - Implementation Details

extension L10n {
  private static func tr(_ table: String, _ key: String, _ args: CVarArg..., fallback value: String) -> String {
    let format = BundleToken.bundle.localizedString(forKey: key, value: value, table: table)
    return String(format: format, locale: Locale.current, arguments: args)
  }
}

// swiftlint:disable convenience_type
private final class BundleToken {
  static let bundle: Bundle = {
    #if SWIFT_PACKAGE
    return Bundle.module
    #else
    return Bundle(for: BundleToken.self)
    #endif
  }()
}
// swiftlint:enable convenience_type
