# Security features – GOV.UK Notify

Quelle: https://www.notifications.service.gov.uk/features/security
Abgerufen: 2026-11-23

```text
   Skip to main content

   Notify

   (BUTTON) Menu
     * Features
     * Pricing
     * Using Notify
     * Support
     * Sign in

    1. Features
         1. Roadmap
         2. Who can use Notify
         3. Security
         4. Performance
    2. Pricing
    3. Using Notify

Security features

   GOV.UK Notify is built for the security needs of government services.

   This page describes our approach to:
     * running a secure service
     * storing and processing your data
     * protecting data in transit
     * protecting data at rest
     * building and managing Notify
     * finding and fixing security issues
     * security incidents
     * sign in and API access
     * protecting our website and API
     * email security
     * security classifications
     * GOV.UK Notify staff
     * suppliers

Running a secure service

   GOV.UK Notify:
     * follows the principles of the Service Standard
     * has been through a successful live service assessment

   We regularly assess and review our security in line with:
     * the GovAssure approach
     * UK government cyber security standards

   We monitor the threat landscape and conduct regular CHECK penetration testing so we can:
     * continue to improve our security
     * deal with common threats like Distributed Denial of Service (DDoS) attacks

Storing and processing your data

   GOV.UK Notify uses Amazon Web Services (AWS) as our cloud service provider.

   Data on Notify is stored and processed in:
     * AWS data centres in the UK and Ireland
     * locations where our sub-processors store and process data (UK and EEA)

How long we keep your data

   GOV.UK Notify keeps a temporary record of:
     * the content of the emails, text messages and letters you send
     * recipient email addresses, mobile numbers and addresses

   By default, we keep this data for 7 days.

   Once your service is live, you can choose the number of days you want Notify to keep details of the messages you send.

   For more information, see data retention period.

Who can access your data

   Your data could be accessed by:
     * the Notify team
     * our sub-processors
     * law enforcement agencies (where legally required)

   Teams using GOV.UK Notify can only access their own data.

   You can set different permissions for each member of your team.

   AWS provides logical separation between different AWS customers.

Data centre security

   AWS provides a description of their:
     * physical security measures, data sanitisation and equipment disposal arrangements
     * security assurance materials

How text messages are stored and processed

   Text messages are stored and processed in:
     * the UK and Ireland
     * the country where the recipient’s phone is
     * the phone’s country of origin (for international numbers)

Protecting data in transit

   GOV.UK Notify uses Transport Layer Security (TLS) version 1.2 to encrypt data when:
     * users access the Notify website or API
     * data passes through Notify
     * we exchange data with our sub-processors

Emails

   We always try to encrypt emails using TLS 1.2, 1.1 or 1.0. If the recipient’s mail server does not support TLS, we will send the email without protection.

   Email cannot provide end-to-end encryption.

Text messages

   Text messages cannot provide end-to-end encryption.

Protecting data at rest

   GOV.UK Notify encrypts the data stored in our databases and backups using AES-256 encryption.

   This includes any files that you upload to Notify when you:
     * send a batch of messages
     * attach pages to a letter template
     * upload a letter

Sending files by email

   When you upload a file we encrypt it with AWS SSE-C, which uses AES-256 encryption.

   We will only share the unique link with the intended recipient. We cannot access or decrypt your file.

   For more information about this feature, see send files by email.

Building and managing GOV.UK Notify

   We follow an Agile software development lifecycle.

   To protect our code, we:
     * run separate development, testing and production environments
     * deploy code through a continuous integration/continuous delivery (CI/CD) pipeline
     * track vulnerabilities for any third-party libraries we use
     * store production secrets in a secure environment with audited access

How we manage code changes

   To manage GOV.UK Notify, we use:
     * GDS-managed devices
     * multi-factor authentication (MFA)

   We manage Notify through the www.notifications.service.gov.uk website.

   AWS manages the hardware we use.

   We use infrastructure as code (IaC) to manage the systems and services that host Notify.

   All code changes must be reviewed by the team before we can deploy them.

   We monitor our production environment for unauthorised changes.

   We give our users appropriate notice before:
     * any planned outages or downtime
     * making significant functional changes

   We announce planned downtime on the Notify status page.

Finding and fixing security issues

   GOV.UK Notify:
     * follows secure development principles
     * tracks third-party dependencies in our code base
     * monitors our logs for attacks, misuse and malfunctions
     * provides 24-hour online support

   We use Web Check and other services to:
     * scan for vulnerabilities
     * prioritise which software patches to test and deploy first

   AWS is responsible for patching our infrastructure:
     * firmware
     * hardware
     * operating system (OS) kernel

Security incidents

   We provide a 24-hour response in case of an incident.

   If there is a data loss event, we will contact you directly.

   If there is another type of incident, we’ll publish details and updates on the Notify status page.

Sign in and API access

Signing in to Notify

   GOV.UK Notify uses two-factor authentication for sign-in.

   Team members can sign in with a text message code or a link that’s sent in an email.

   For security, you’ll need to confirm that you still have access to your email address every 3 months.

   Find out more about our sign-in methods.

   You must keep to our terms of use for signing in to Notify.

Accessing the GOV.UK Notify API

   Services access the GOV.UK Notify API with an API key, encoded using JSON Web Tokens.

   For more information, see our API documentation.

Protecting our website and API

   The GOV.UK Notify website, API and any files sent by email are protected by:
     * AWS Web Application Firewall (WAF)
     * AWS Shield Advanced
     * rate limiting

   We use publicly-verifiable digital certificates, so you’ll always know you’ve connected to the real GOV.UK Notify.

Email security

   To help recipient’s email services tell the difference between our emails and spam, we use:
     * Domain-based Message Authentication, Reporting and Conformance (DMARC)
     * DomainKeys Identified Mail (DKIM)
     * Sender Policy Framework (SPF)

Security classifications

   We have designed GOV.UK Notify for sending messages classified as ‘OFFICIAL’, including ‘OFFICIAL-SENSITIVE’, under the Government Security Classifications policy.

   Before you send any messages classified as ‘OFFICIAL’, you must make sure that GOV.UK Notify meets your organisation’s standards for:
     * using, processing, storing and sending information
     * cyber security
     * data protection

   Notify must not be used to process data classified as ‘SECRET’ or ‘TOP SECRET’.

GOV.UK Notify staff

   We restrict the number of people that can access your data on GOV.UK Notify.

   We follow the principle of least privilege. This means we give our team members the lowest level of permissions needed to do their job.

   All our staff:
     * receive security training
     * complete personnel screening equivalent to BPSS

   Team members who need greater access to the data stored on Notify must complete National Security Vetting to Security Check (SC) level.

   We only give additional access to GOV.UK Notify’s production environment to privileged users:
     * by exception
     * on a temporary basis
     * in relation to a specific change request or support ticket

   The GDS security operations team logs and tracks privileged users’ access to our production environment.

Suppliers

   GOV.UK Notify uses third-party providers to send emails, text messages and letters.

   Suppliers sign a contract or memorandum of understanding that includes our security requirements.

   All our suppliers:
     * receive security training
     * complete personnel screening equivalent to BPSS

   GDS assesses suppliers:
     * before we decide whether to use them
     * at regular intervals to make sure they still meet our requirements

About Notify

     * Features
     * Pricing
     * Using Notify
     * Blog

Support

     * Contact support
     * Chat to us on Slack
     * System status
     * API documentation
     __________________________________________________________________________________________________________________________________________________________________________________________

Support links

     * Privacy
     * Accessibility statement
     * Terms of use
     * Cookies

   Built by the Government Digital Service
   All content is available under the Open Government Licence v3.0, except where otherwise stated
   © Crown copyright
```
