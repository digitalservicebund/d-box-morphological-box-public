<!--
SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)

SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Glossary

> [!NOTE] Adding glossary terms
> We discuss glossary item definition in our
> [glossary room](https://matrix.to/#/#glossary.neo:fitko.dev).
> Please file the need for new glossary items via the following template there:
>
> ```md
> ### New glossary term: <term that you would like to add to the glossary>
> 
> * Alternative terms / abbreviations (English): 
> * Alternative terms / abbreviations (German): 
> * Definition / Definition of the term (English): 
> * Links / sources: 
> ```

| Term | Alternative terms | German term(s) | Definition |
| ---- | ----------------- | -------------- | ---------- |
| Administrative act | official decision | Verwaltungsakt | An administrative act is legally defined in Section 35 of the Administrative Procedure Act ([VwVfG](https://www.gesetze-im-internet.de/vwvfg/__35.html)). It includes any order, decision or other measure taken by a public authority to regulate an individual case in the field of public law and which has a direct legal effect on external parties. |
| Administrative decision | official letter | Bescheid | An administrative decision is an official document containing a specific, individual order or finding issued by an administrative authority or a court. Under German administrative law, an administrative decision must contain at least one administrative act. |
| ADR | Architecture Decision Record | Dokumentierte Architekturentscheidung | An Architectural Decision (AD) is a justified software design choice that addresses a functional or non-functional requirement of architectural significance. An Architectural Decision Record (ADR) captures a single AD and its rationale. ([source](https://adr.github.io/)) |
| Advanced electronic signature | | Fortgeschrittene elektronische Signatur | An advanced electronic signature is an electronic signature that meets the requirements of Article 26 of the eIDAS Regulation (source: [Article 3, eIDAS Regulation](https://eur-lex.europa.eu/eli/reg/2024/1183/oj/eng)). An advanced electronic signature meets all of the following requirements: a) It is uniquely linked to the signatory. b) It enables the identification of the signatory. c) It is created using electronic signature creation data that the signatory can use with a high level of confidence under their sole control. d) It is linked to the data signed in such a way that any subsequent change to the data can be detected. |
| Application | | Antrag | The term application describes the initial request for a public service usually sent using an online service. |
| Authentication | | Authentifizierung | Authentication is the act of verifying a subject's identity. (source: [BSI TR-03160-1](https://www.bsi.bund.de/SharedDocs/Downloads/DE/BSI/Publikationen/TechnischeRichtlinien/TR03160/BSI-TR-03160-1.html)) |
| Authenticity | | Authentizität | The term authenticity refers to the property of being genuine and able to be verified and be trusted ("authentic"). In Neo, we refer to a communication channel as an 'authenticated channel' if all participants have proven their identity to each other. |
| Case | | Vorgang, Verwaltungsverfahren | The term case refers to the activity of a competent agency aimed at handling an application (Antrag), as well as preparing and issuing an administrative act. In our setting, all communication between a private individual and the competent agency is always part of a specific case. (source: [§ 9 VwVfG](https://www.gesetze-im-internet.de/vwvfg/__9.html)) |
| Competent agency | competent authority, public authority, government agency | Fachbehörde, Fachamt | Any body or authority at national, regional, or local level with specific responsibilities for at least one public service (usually provided online). (source: [SDG regulation](https://eur-lex.europa.eu/eli/reg/2018/1724/oj/eng#art_3)) |
| Confidential metadata | | Vertrauliche Metadaten | Confidential metadata refers to all metadata that is not relevant for transport and therefore must be protected by end-to-end encryption or other means. Examples: tax ID, identity card ID. |
| Device Verification | | | The act of cryptographically proving that a Matrix device is actually operated by their respective owner. In Neo, private individuals can do so by using the eID functionality or their BundID account. Government authorities prove their identity by using an X.509 certificate. This term is defined by the [Matrix specification](https://spec.matrix.org/latest/client-server-api/#device-verification). |
| European Digital Identity Wallet | EUDI-Wallet | Europäische Brieftasche für Digitale Identität | The ‘European Digital Identity Wallet’ is an electronic identification tool that enables users to securely store, manage and validate personal identification data and electronic attribute certificates in order to present them to relying parties and other users of European Digital Identity Wallets and to sign them using qualified electronic signatures or seal them using qualified electronic seals. (in accordance with [eIDAS Regulation 2.0](https://www.digitale-verwaltung.de/Webs/DV/DE/digitale-identitaeten/eidas-2-0/eidas-2-0.html)) |
| IT system of the competent agency | | Fachverfahren, Fachdienst | IT systems of competent agencies are used by competent agencies to process private individuals' applications and respond to their requests. IT systems of competent agencies usually serve to fulfil one or more specific government service but may also be designed in a generic way to handle various government services. An IT system of the competent authority can consist of various components, such as databases, software applications and interfaces that enable data to be exchanged between different systems and users. |
| Mailbox | Inbox | Postfach | A mailbox is a functional component that enables \[users\] to compose and send electronic correspondence, as well as to receive, store and display it. |
| Means of Authentication | | Authentisierungsmittel, Identifizierungsmittel | Authentication means are technical means that allow the holder to authenticate an identity (that is, a set of identity attributes) or other transmitted data. Examples of authentication means include passwords, identity cards, or cryptographic tokens (source: [BSI TR-03107-1](https://www.bsi.bund.de/SharedDocs/Downloads/DE/BSI/Publikationen/TechnischeRichtlinien/TR03107/TR-03107-1.html)) |
| MSC | Matrix Spec Change | | A proposal to change the Matrix specification. (see [Matrix specification](https://spec.matrix.org/proposals/)) |
| Online service | | Onlinedienst, OD | An online service is an IT system providing a user interface to request some public service via the internet. (source: [§ 2 OZG](https://www.gesetze-im-internet.de/ozg/__2.html)) |
| Private Individual | Citizen | Privatperson, Bürger:in | A natural person who requests a public service. Since the term 'citizen' usually implies citizenship to a sovereign state which is not a requirement to request some public service, the term 'private individual' is more generic and often more correct. Still, we sometimes use the term 'citizen' as a synonym. |
| Qualified electronic signature | | Qualifizierte elektronische Signatur | Qualified electronic signature is an advanced electronic signature created by a qualified electronic signature creation device and based on a qualified certificate for electronic signatures. ([source](https://www.elektronische-vertrauensdienste.de/EVD/DE/Uebersicht_eVD/Dienste/1_Signatur.html?nn=691386)) |
| Shared mailbox | | Funktionspostfach | Several users in the role mailbox members as authorised representatives have access to the functions of a shared mailbox. |
| Synapse | | | [Synapse](https://github.com/element-hq/synapse/) is an open source Matrix homeserver implementation, written and maintained by Element. |
| Users | | Nutzer:innen | Users in the context of Neo are private individuals and competent agencies. |
| WCAG | Web Content Accessibility Guidelines | | [International standard](https://www.w3.org/WAI/standards-guidelines/wcag/) that explains how to make web content more accessible. |
| ZaPuK | | Zielarchitektur Postfach- und Kommunikationslösung | Target architecture for mailbox and communication solution. ([source](https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk)) |
| ZBP | Central Citizen Mailbox | Zentrales Bürgerpostfach | The Central Citizen Mailbox ([__ZBP__](https://docs.fitko.de/fit-connect/docs/organisation-tasks/buergerpostfach-entscheider/)) is BundID's current-generation mailbox infrastructure that enables IT systems of competent agencies to send administrative decisions to private individuals. It provides a RESTful API but does not implement end-to-end encryption. |
