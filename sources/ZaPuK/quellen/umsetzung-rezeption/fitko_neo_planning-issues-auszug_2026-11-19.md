# FITKO Neo: Planning-Issues zu Transition, Protocol Converter, Benachrichtigung, Authentifizierung (Auszug)

Quelle: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/issues (GitLab-API, je Issue unten verlinkt)
Abgerufen: 2026-11-19

Auswahl relevanter Issues des öffentlichen Neo-Issue-Trackers. Beschreibungstexte unverändert (Englisch), eingebettete Bilder nicht übernommen. Arbeitsstände, keine beschlossenen Dokumente.

## #715 [Epic] MJP/EGVP Integration

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/715
- Status: opened, angelegt 2026-07-20, zuletzt geändert 2026-10-01
- Labels: EPIC, Needs pre-refinement, Prio::0, Scheduled-for-refinement-meeting

> > [!note]
> >
> > This epic is reflected as a milestone in https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/milestones/73
>
> ## Meta-Story
>
> As a citizen, I want to authenticate via eID and access my court and justice authority messages directly through Neo, and as a justice authority, I want to continue using the existing EGVP infrastructure to send and receive messages, so that citizens and courts can communicate seamlessly across the Neo and EGVP infrastructures.
>
> According to ZaPuK, we want to establish a [protocol converter](https://youtu.be/s5BrVVf0B1I?t=1658) between our Matrix Federation and the [EGVP](https://egvp.justiz.de/) infrastructure.
>
> See red rectangle:
>
> ![grafik](/uploads/a3632abdf8c5496da0b12575fb87ff6e/grafik.png){width="900" height="487"}
>
> ## Stakeholder value
>
> * Citizens gain access to court and justice authority communication directly through Neo and BundID via eID authentication, thus eliminating the need to register separately for MJP and enabling a unified mailbox experience for all government communication including Justiz messages.
> * Justice authorities and courts gain a protocol converter that bridges their existing EGVP/OSCI infrastructure with the Neo Matrix federation, allowing them to continue using established tools and workflows without replacing them.
> * ZaPuK's goal of a central mailbox frontend that replaces parallel UIs from BundID, MJP, and MUK progresses.
>
> ## Scope
>
> - ~~UX/ UI desk research on Justizpostfach approach.~~ -\> %&quot;UX/UI for organizations&quot;
> - Talk to Element about bridging.
> - Architecture concept about message formats / representation in Matrix
> - Protocol converter between OSCI and Matrix.
> - XML decryption and encryption in Neo web client (including receiving and sending messages).
> - XML decryption and encryption in Neo iOS app.
> - XML decryption and encryption in FIT Connect SDK.
> - Searching and addressing justice organizations.
>
> This epic addresses the integration between the Neo Matrix federation and the EGVP infrastructure used by German courts and justice authorities. EGVP uses the OSCI protocol as secure message transport standard, which is incompatible at the protocol used within Matrix. A protocol converter must therefore be developed that translates between OSCI and Matrix without breaking E2EE.
>
> The OSCI library integration, the setup of the protocol converter repository, and hands-on experience with the OSCI test intermediary are addressed here as foundational steps. The architecture concept for message formats and their representation in Matrix is equally within scope, as is the evaluation of bridging options in collaboration with Element. Retrieving a message from the EGVP world and delivering it as an encrypted message in a Matrix room is a deliverable of this epic, and can be seen as PoC.
>
> ~~A desk research into the existing Mein Justizpostfach UX/UI approach, including documenting it and identifying possible new Neo requirements is also within this scope.~~ -\> Part of %&quot;UX/UI for organizations&quot;
>
> ## Requirements
>
> Justiz requirements tbd...
>
> ## Completion Criteria
>
> * [ ] The OSCI library is integrated and a connection to the OSCI test intermediary is established.
> * [ ] A message can be retrieved from the EGVP and delivered as an encrypted message in a Matrix room without breaking E2EE.
> * [ ] The architecture concept for EGVP specific message formats and their integration in Matrix is documented.
> * [ ] An document describing the architecture decision on the protocol converter is produced.
> * [ ] ~~The UX/UI research on the existing Mein Justizpostfach solution is documented.~~
> * [ ] Whether additional FIT-Connect SDK work is required for the EGVP integration is investigated.
> * [ ] An approach for the Matrix client integration into the Justiz infrastructure is defined.
>
> ## Notes / resources
>
> - https://egvp.justiz.de/buerger_organisationen/index.php
> - Transitionsbetrachtung: Mein Justizpostfach (MJP)
> - TADR-MJP-001: Anbindungsmodell von MJP an ZaPuK
> - @\<SAFE_ID\>:matrix.justiz.de
>
> ### Architecture View
>
> ![grafik](/uploads/6e9dcbe79ed6dc1cd0c7330c54b042fb/grafik.png){width="900" height="258"}
>
> ## Who needs to be involved / informed
>
> * involved:
> * informed:

## #717 Protocol converter: Explore OSCI integration

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/717
- Status: opened, angelegt 2026-07-20, zuletzt geändert 2026-10-07
- Labels: 5 Story Points, In Review, Prio::0, Sprint 23, Sprint 24, Sprint 25, team::dev

> ## Motivation/Context
>
> According to ZaPuK, we want to establish a [protocol converter](https://youtu.be/s5BrVVf0B1I?t=1658) between our Matrix Federation and the [EGVP](https://egvp.justiz.de/) infrastructure.
>
> See red rectangle:
>
> ![grafik](/uploads/a3632abdf8c5496da0b12575fb87ff6e/grafik.png){width=900 height=487}
>
> As a first step, we want to get hands-on experience with OSCI protocol.
>
> ## Scope
>
> - ~~Setup [OSCI library](https://www.governikus.de/service/osci-bibliothek/) and connect to OSCI test intermediary~~
> - NEW: Implement OSCI primitives in Rust
> - Thereby lay foundations for Matrix <-> OSCI protocol converter
>
> Out of scope:
> - Implementing Matrix protocol
> - Interpretation of any message contents
>
> **Time box:** 5 days
>
> ## Result/ACs
>
> - [ ] A new repository ~~based on our template repo~~ is created. => osci-transport-rs
> - [ ] ~~Results are documented as source code and log output.~~ (we irgore AC as of 2026-09-30)
> - [ ] ~~An brief ADR draft is created about the choice of the programming language being used for our protocol converter.~~ (we irgore AC as of 2026-09-30)
> - [ ] NEW: Implementation of OCSI to the extend we need it
> - [ ] Reviewed by @evlli 
>
> ## Notes/Hints/Ideas
>
> - Spec: https://www.xoev.de/sixcms/media.php/13/osci_entwurfsprinzipien_1_2.pdf
> - Whereas a bridge **decryptes message contents** while converting from protocol A to protocol B, a protocol converter **does not break end-to-end-encryption** while converting from protocol A to protocol B. See slide 25 of https://docs.google.com/presentation/d/1hxRlyJ6SD3zXb2MPGmfigB0s9Yj4dsWBMtctNPv06IA/edit?slide=id.g2b639286114_0_142#slide=id.g2b639286114_0_142.
>
> ## Who needs to be involved / informed
>
> - reviewer: 
> - involved: 
> - informed:

## #750 MJP Protocol converter: Bridge Messages to Matrix Room

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/750
- Status: opened, angelegt 2026-08-10, zuletzt geändert 2026-11-18
- Labels: 5 Story Points, In Progress, Sprint 27, Sprint 28, team::dev

> ## Problem
>
> We want to build a protocol converter that creates interoperability between ZaPuK (Matrix) and EGVP/MJP (OSCI).
>
> ## Goal
>
> Proof-of-concept: Using our OSCI library, a message is retrived via OSCI and sent to a Matrix room.
>
> Out-of-scope
> - Any implementation in the web client is out of scope
> - Decrypting the inner encryption layer of the message
> - OSCI signature checks
> - Matrix e2ee
> - Handling of multiple Matrix accounts (appservice etc.)
>
> ## ACs
>
> - [ ] A new repository based on our template repo is created
> - [ ] An OSCI message is XML-transport-decrypted and sent to a Matrix room by the bridge.
> - [ ] The inner encryption layer of the message is kept intact.
> - [ ] The encrypted messages is visible in Element Web (potentially after enabling the option to show hidden events in /devtools)
> - [ ] Reviewed by ~"team::dev" 
>
> ## Requirements
>
> \-
>
> ## Notes / Resources
>
> - [ ] Validate which encryption /signature schemes are used by MJP in production
>
> ## Who needs to be involved / informed
>
> - reviewers:
> - involved:
> - informed:

## #798 MJP integration: Decrypt and convert XJustiz to Neo message format

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/798
- Status: opened, angelegt 2026-09-30, zuletzt geändert 2026-11-18
- Labels: 5 Story Points, Ready for Sprint, Sprint 27, Sprint 28, team::dev

> ## Problem
>
> Messages from MJP have a different structure from what we need in Matrix events. We cannot get the contents before having them transported over the bridge as they are encrypted.
>
> ## Goal
>
> Decrypt OSCI messages and render them in a matrix client. We only care about PDF-type attachments messages.
>
> ## ACs
>
> todo: Which XJustiz message format?
>
> - [ ] ...
> - [ ] osci-transport-ffi library is integrated in web client
> - [ ] Message is rendered in web client
> - [ ] iOS? -> seperate issue @mgcm will create it once we have more insight from web impl
> - [ ] Reviewed by 
>
> ## Requirements
>
> - [ ] Provide requirement(s) UID(s) from requirements table.
>
> ## Notes / Resources
>
> > additional info that is helpful when working on the ticket
> > images, Figma, docs, etc
>
> - 
>
> ## Who needs to be involved / informed
>
> - reviewers: 
> - involved: 
> - informed:

## #801 Evaluate Implications of MJP Integration for Private Individuals

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/801
- Status: opened, angelegt 2026-10-01, zuletzt geändert 2026-11-18
- Labels: 5 Story Points, In Progress, Sprint 26, Sprint 27, Sprint 28, team::ux

> ## Problem
>
> We have requirements from MJP but have only added them to the organizations case. We have not analyzed the requirements and the implications of the MJP integration for private individuals.
>
> ## Goal
>
> Have a well-founded understanding and documentation of the implications of integrating MJP in Neo for citizens. What do we have to adjust in the design and funtionalities
>
> ## ACs
>
> - [x] Apply requirements of MJP to Neo for private indivuduals
> - [x] Make a low-fi proposal of how to solve these implications
> - [ ] hand over these changes to UI Design (by creating one or multiple issues around this topic)
>
> ## Requirements
>
> n.a.
>
> ## Notes / Resources
>
> Implications, we already know, we need to consider:
>
> * ZIP Download for citizens of the whole case
> * Download of xJustiz on each messages
> * Adjustment of Prüfvermerk
>
> We need to validate these requirements by understanding their real-world use-cases.
>
> https://nextcloud.fitko.de/apps/files/files/3359380?dir=/Messenger-Kommunikation/UX%26UI/Unternehmenskonto&editing=false&openfile=true
>
> ## Who needs to be involved / informed
>
> - reviewers:
> - involved:
> - informed:

## #17 [Epic] Bridge for existing ZBP messages

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/17
- Status: opened, angelegt 2025-06-25, zuletzt geändert 2026-11-18
- Labels: EPIC, Needs pre-refinement, Prio::High, Scheduled-for-refinement-meeting, team::architecture, team::dev

> > [!note]
> >
> > This epic is reflected as a milestone in https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/milestones/31
> > 
> > Mirror-Epic in FIT-Connect: https://git.fitko.de/groups/fit-connect/-/work_items/8
>
> ## Meta-Story
>
> As the Federal IT Architecture Board (FIT-AB) and as the ZaPuK project, I want to understand how a migration strategy from the existing ZBP to a Matrix-based infrastructure could look like, so that I have a good basis for decision-making and demonstrating how all existing messages, documents, and communication history will remain fully accessible after the migration, while the transition to Neo remains invisible and no data or communication continuity is lost.
>
> ## Stakeholder value
>
> * The Federal IT Architecture Board and ZaPuK gain a documented, evaluated migration strategy that provides a solid basis for decision-making about the transition from ZBP to Matrix-based infrastructure.
> * Citizens gain seamless continuity of access to all their existing BundID postbox messages, documents, and Bescheide through Neo without needing to take any action or even notice that a migration has occurred.
> * Competent authorities gain compatibility and a bridge that ensures that they can continue communicating with citizens without disruption while the infrastructure migrates.
>
> ## Scope
>
> This epic addresses the design and evaluation of a migration strategy from the existing ZBP to the Neo Matrix-based infrastructure, and the development of a bridge between the ZBP postbox and the Matrix homeserver that makes this migration technically possible. The bridge must enable bidirectional synchronization between ZBP and Matrix. The NdB-VN network connection to the Matrix is also to be defined and documented as part of this epic.
>
> The migration concept covers all aspects of a safe and compliant migration: data mapping, backup strategy, rollout planning, downtime windows, simulation and testing phases, data integrity validation, GDPR compliance, and a documentation and support strategy for competent authorities. Risk assessment and the definition of specific measures and tools are equally within scope.
>
> In-scope:
>
> - Defining and evaluating a migration strategy.
> - Evaluating the strategy from a UX perspective (private individuals).
> - Evaluating the strategy from a developer experience perspective (government agencies).
> - Evaluating the strategy from an operation perspective (BundID, FIT-Connect).
> - Defining expected behaviour when dealing with different data formats.
>
> Out-of-scope:
>
> - Performing the actual migration
>
> ## Transition plan
>
> ### Transition Step 0: Matrix Homeservers are operational - no integration yet
>
> ![grafik.png](/uploads/6a26558cd2d5aa187c0b3ca95f824b1a/grafik.png){width=900 height=294}
>
> ### Transition Step 1: Data Interoperability Pilot - for research purposes only
>
> ![grafik.png](/uploads/34ba790eec4d3d8fa2f18e7e8a487e5b/grafik.png){width=900 height=294}
>
> ### Transition Step 2: Bridge replaces ZBP backend - Data is migrated - ZBP Backend
>
> ![grafik.png](/uploads/4e2318f48c4a8fe952ee36533a1d9162/grafik.png){width=900 height=294}
>
> ### Transition Step 3: Bridge is decomissioned - All Govermnent IT Systems are migrated
>
> ![grafik.png](/uploads/bc4e57c2b80d353e8e630d33ad886326/grafik.png){width=900 height=294}
>
> ## Issues
>
> Issue that belong to this Epic.
>
> - #120+
> - #106+
> - #108+
> - #107+
>
> ## Completion
>
> * [ ] A concept for migration must be created and approved.
> * [ ] The solution handles all new and existing message flows:
>   * [ ] Existing messages that have already been received by the ZBP are displayed in the Neo web client and the Neo app
>   * [ ] New messages that will be received in the future by the ZBP are displayed in the Neo web client and the Neo app
>   * [ ] New and existing messages that have been received via the Matrix home server for governments are displayed in the Neo web client and the Neo app
>   * [ ] New and existing messages that have been received via FIT-Connect (via the ZBP adapter) are displayed in the Neo web client and the Neo app
>   * [ ] As a private individual, I am able to to answer to each of these messages (if message type allows bidirectional communication).
> * [ ] A proposal for a roadmap is defined that includes a plan for decommissioning old APIs and infrastructure
>
> ## Requirements
>
> - LB_49: Both internal and external dependencies of the software components must be described, including technical dependencies (e.g., external APIs, libraries) and organizational dependencies (e.g., dependencies between development teams).
> - LB_138: The migration concept ensures that a seamless transition from the current technical status (ACTUAL) to the new, piloted target architecture is possible.
> - LB_139: All existing communication content should remain transparent and continuously available.
> - LB_140: The migration concept should serve as the basis for subsequent productive implementation (the implementation of the migration is not part of the scope of the contract).
> - LB_141: The migration concept will be written in English.
> - LB_142: The concept must address the realization of seamless access for end users to existing and new communication content.
> - LB_143: The migration must be "invisible" to end users.
> - LB_144: Existing and new communication content should be seamlessly accessible in a uniform user interface.
> - LB_145: Existing content from the previous infrastructure should remain available via web-based access and, ideally, also be accessible via the new app-based access.
> - LB_146: The concept must identify technical solution options that enable this dual access, e.g., through data integration layers or APIs for data aggregation.
> - LB_147: Ensuring compatibility and support for specialist authorities.
> - LB_148: (Related to LB_147) assist specialist authorities in adapting and integrating the new architecture into their IT systems.
> - LB_149: (Related to LB_147) If necessary, provide the necessary interfaces or develop adapter solutions such as protocol converters to enable the connection of their existing systems.
> - LB_150: (Related to LB_147) The migration concept should describe specific measures and tools.
> - LB_151: Technical and organizational measures for successful migration: The migration concept should describe all other known technical and organizational steps necessary for a smooth transition.
> - LB_152: Technical and organizational measures for successful migration: Ensuring data consistency between systems.
> - LB_153: Technical and organizational measures for successful migration: Planning a migration schedule.
> - LB_154: Technical and organizational measures for successful migration: defining test and validation procedures.
> - LB_155: Technical and organizational measures for successful migration: Confirm data integrity during and after migration.
> - LB_156: Risk assessment and measures to ensure data integrity: Identifying potential risks and challenges in the migration process.
> - LB_157: Risk assessment and measures to ensure data integrity: Defining measures to ensure data integrity.
> - LB_158: Risk assessment and measures to ensure data integrity: The concept should include backup and recovery procedures.
>
> ## Notes / resources
>
> ## Who needs to be involved / informed
>
> - involved:
> - informed:

## #120 Migration concept: Designing a bridge between ZBP and Matrix server

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/120
- Status: opened, angelegt 2025-07-24, zuletzt geändert 2026-08-24
- Labels: Backlog, Prio::High, Ready for Refinement, team::architecture

> **Problem:**
>
> - According to the [requirements catalogue](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md?ref_type=heads#313-erstellung-eines-migrationskonzepts) and the workshop day 3 protocol, a bidirectional migration concept must be designed.
> - Therefore, a bridge or 'adapter' must be built to enable communication between the ZBP postbox and the Matrix server.
> - The concept of how the ZBP\<-\>Matrix homeserver communication will take place still needs to be developed (see docs below).
>
> **Goal:**
>
> Determine how the ZBP postbox will communicate with the Matrix homeserver, thus allowing data migration.
>
> **ACs:**
>
> As citizen or a public agent accessing my BundID postbox, now upgraded by Neo to be a matrix-based bi-directional communication, I want to:
>
> * [ ] see all my previous postbox messages without any changes, errors or delay.
> * [ ] see all my previous uploaded documents and shared data, as well as received Bescheide.
> * [ ] as always, be able to contact and be contacted by the public agencies to which I have submitted my requests.
> * [ ] not notice when and if the migration happened.
> * [ ] Reviewer
>
> **Requirements:**
>
> * [ ] LB_139 : All existing communication content should remain transparent and continuously available.
> * [ ] LB_140 : The migration concept should serve as the basis for subsequent productive implementation (the implementation of the migration is not part of the scope of the contract).
> * [ ] LB_142 : The concept must address the realization of seamless access for end users to existing and new communication content.
> * [ ] LB_143 : The migration must be “invisible” to end users.
>
> **Note/ resources:**
>
> - ZBP interfaces description attached : [ZBP_Schnittstellenbeschreibung_Zentrales_Bürgerpostfach_H3.0.0.0_V4.0.pdf](/uploads/54e25763af1c5ea7d6c65ed3a99b02c5/ZBP_Schnittstellenbeschreibung_Zentrales_Bu%CC%88rgerpostfach_H3.0.0.0_V4.0.pdf)
> - BundID postbox certificates attached : [BundID-Postfach-Zertifikat_Anleitung_V5.0.pdf](/uploads/2a4c364ddc8e5cfb140c0f8d410ad624/BundID-Postfach-Zertifikat_Anleitung_V5.0.pdf)
> - Design concept (maybe for another ticket):
>   * Read ZBP postbox documents attached.
>   * Build a diagram depicting the possible communication flow with the Matrix server.
>   * Write down open points, if any.
>   * Decide if we want to implement E2EE between ZBP bridge and app/web (see notes in #106)
>   * Document diagram and open points in this ticket.
>   * Document diagram in Nextcloud.
>   * Clarify with stakeholders how to proceed.

## #504 ZBP Migration Concept

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/504
- Status: opened, angelegt 2026-02-26, zuletzt geändert 2026-08-24
- Labels: Backlog, Needs pre-refinement, Prio::High, team::architecture, team::requirements

> ## Motivation/Context
>
> Citizen and public agencies using the BundID postbox will have their messages, data and documents migrated to a Matrix homeserver.
>
> ## Scope
>
> * Read ZBP postbox documents attached.
> * Evaluate and describe plan of migration (source databank type, data to be migrated, downtime window, backup strategy, rollout plan, date and time, etc.)
>   * Identify open points, if any.
> * Conduct full backup.
> * Data mapping.
> * Simulation, testing phase, validation measures, proving data integrity.
> * Monitor migration logs and error events.
> * Guarantee compliance with GDPR.
> * Documentation and support strategy for competent authorities.
> * Describe specific measures and required tools.
> * Risk assessment.
>
> **Time box:**
>
> ## Result/ACs
>
> - A fully functional ZPB\<-\>Matrix homeserver bridge exists, proving that migrating data is possible, secure, and safe.
>
> ## Requirements
>
> * [ ] LB_49 : Both internal and external dependencies of the software components must be described, including technical dependencies (e.g., external APIs, libraries) and organizational dependencies (e.g., dependencies between development teams).
> * [ ] LB_138 : The migration concept ensures that a seamless transition from the current technical status (ACTUAL) to the new, piloted target architecture is possible.
> * [ ] LB_141 : The migration concept will be written in English.
> * [ ] LB_144 : Existing and new communication content should be seamlessly accessible in a uniform user interface.
> * [ ] LB_145 : Existing content from the previous infrastructure should remain available via web-based access and, ideally, also be accessible via the new app-based access.
> * [ ] LB_146 : The concept must identify technical solution options that enable this dual access, e.g., through data integration layers or APIs for data aggregation.
> * [ ] LB_147 : Ensuring compatibility and support for specialist authorities.
> * [ ] LB_148 : (Related to LB_147) assist specialist authorities in adapting and integrating the new architecture into their IT systems.
> * [ ] LB_149 : (Related to LB_147) If necessary, provide the necessary interfaces or develop adapter solutions such as protocol converters to enable the connection of their existing systems.
> * [ ] LB_150 : (Related to LB_147) The migration concept should describe specific measures and tools.
> * [ ] LB_151 : Technical and organizational measures for successful migration: The migration concept should describe all other known technical and organizational steps necessary for a smooth transition.
> * [ ] LB_152 : Technical and organizational measures for successful migration: Ensuring data consistency between systems.
> * [ ] LB_153 : Technical and organizational measures for successful migration: Planning a migration schedule.
> * [ ] LB_154 : Technical and organizational measures for successful migration: defining test and validation procedures.
> * [ ] LB_155 : Technical and organizational measures for successful migration: Confirm data integrity during and after migration.
> * [ ] LB_156 : Risk assessment and measures to ensure data integrity: Identifying potential risks and challenges in the migration process.
> * [ ] LB_157 : Risk assessment and measures to ensure data integrity: Defining measures to ensure data integrity.
> * [ ] LB_158 : Risk assessment and measures to ensure data integrity: The concept should include backup and recovery procedures.
>
> ## Notes/Hints/Ideas
>
> - ZBP interfaces description attached : ZBP_Schnittstellenbeschreibung_Zentrales_Bürgerpostfach_H3.0.0.0_V4.0.pdf
> - BundID postbox certificates attached : BundID-Postfach-Zertifikat_Anleitung_V5.0.pdf
>
> Draft:
>
> > Zur Gewährleistung einer sicheren, verschlüsselten und bidirektionalen Kommunikation zwischen Privatpersonen und Fachbehörden werden bestehende Nachrichten, Daten und Dokumente aus dem Zentralen Bürger:innen-Postfach (ZBP) auf einen Matrix-Homeserver migriert. Eine Migrationsstrategie wird erstellt, um bestehende Daten aus dem ZBP in eine Matrix-basierte Infrastruktur zu übertragen. Vor der Migration wird das Konzept in einer Testphase validiert, in der die voll funktionsfähige Brücke zwischen dem ZBP und dem Matrix-Homeserver bewertet wird. Ziel ist es, zu bestätigen, dass die Datenmigration sowohl technisch machbar als auch sicher ist.
> >
> > Für einen Übergangszeitraum werden Nachrichten aus dem ZBP in den Matrix-Homeserver bidirektional synchronisiert, sodass alle über das ZBP gesendeten bzw. empfangenen Nachrichten auch über die Matrix-basierte Infrastruktur zur Verfügung stehen. Fachbehörden, die sich bereits an das ZBP angebunden haben, können in dieser Übergangsphase über das ZBP mit Nutzenden des Neo-Webclient sowie der Neo-Smartphone-App kommunizieren, ohne ihre Fachverfahrensschnittstellen anpassen zu müssen. Für die Umsetzung einer Ende-zu-Ende-Verschlüsselung bis in das Fachverfahren der Fachbehörde hinein ist hingegen eine Migration auf das von Neo bereitgestellte SDK notwendig (Anpassung des Fachverfahrens).
> >
> > Das vollständige Migrationskonzept (nicht Teil von Phase III des Neo-Vorhabens) wird mindestens den folgenden Umfang abdecken:
> >
> > - Analyse der bestehenden ZBP-Technologie und -Architektur.
> > - Bewertung und Dokumentation des Migrationsplans einschließlich Quelldatenbanktyp, zu migrierender Daten, Wartungsfenster, Backup-Strategie, Rollout-Plan sowie geplantem Datum und Uhrzeit.
> > - Vollständiges Backup vor der Migration.
> > - Datenmapping zwischen Quell- und Zielsystem.
> > - Simulation, Testdurchläufe und Validierungsmaßnahmen zur Sicherstellung der Datenintegrität.
> > - Überwachung von Migrationsprotokollen und Fehlerereignissen.
> > - Sicherstellung der DSGVO-Konformität während des gesamten Migrationsprozesses.
> > - Dokumentation und Support-Strategie für zuständige Behörden.
> > - Definition konkreter Maßnahmen und erforderlicher Werkzeuge.
> > - Risikobewertung.
> >
> > **_Anwendungsfall:_** BundID-Nutzer wollen die neue bidirektionale und sichere Kommunikationsinfrastruktur sowie die Funktionalitäten von BundID-Neo benutzen. Dafür müssen sich deren Daten zunächst im Matrix-Homeserver befinden. Dementsprechend müssen die Daten aus ZBP durch eine Brücke zum Matrix-Homeserver migriert werden.
> >
> > - _Privatperson:_ Als Privatperson will ich während und nach der Migration keine Unterbrechung meines Postfachs wahrnehmen. Konkret will ich: alle bisherigen Postfachnachrichten unverändert, fehlerfrei und ohne Verzögerung einsehen können; alle zuvor hochgeladenen Dokumente, geteilten Daten sowie erhaltenen Bescheide weiterhin abrufen können; weiterhin Kontakt mit den Behörden aufnehmen und von diesen kontaktiert werden können, bei denen ich Anträge gestellt habe; und die Migration nicht bemerken.
> > - _Behörde:_ Als Sachbearbeiter will ich während und nach der Migration keine Unterbrechung meines Postfachs wahrnehmen. Konkret will ich: alle bisherigen Postfachnachrichten unverändert, fehlerfrei und ohne Verzögerung einsehen können; alle zuvor enthaltenen Dokumente, empfangenen Daten sowie zugestellten Bescheide weiterhin einsehen können; weiterhin Kontakt mit Privatpersonen aufnehmen können, die Anträge gestellt haben, und von diesen kontaktiert werden können; und die Migration nicht bemerken.
>
> ## Who needs to be involved / informed
>
> - reviewer:
> - involved:
> - informed:

## #607 Import messages via BundID Frontend API

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/607
- Status: opened, angelegt 2026-05-15, zuletzt geändert 2026-10-07
- Labels: 5 Story Points, In Progress, Prio::High, Sprint 23, Sprint 24, Sprint 25, Sprint 26, team::dev

> ## Motivation/Context
>
> In order to test interoperability between ZBP and Neo, we want to import ZBP messages to a user's Matrix account (using the message formats defined by Neo).
>
> ## Scope
>
> - Import messages from a user's ZBP account to the user's Matrix account ("one-shot").
>   - -> this implies there is an MXID for the ZBP used as "sender" in Matrix
> - Respect the message formats defined by Neo
>
> ### Out of scope
>
> - Running a service that continuously bridges messages.
> - Pushing replies from Matrix to ZBP (bidirectionality).
>
> **Time box: 20 working hours**
>
> ## Result/ACs
>
> * [ ] ~~The bridge provides (a simple web frontend | a simple CLI) that allows to import messages from ZBP to the users' Matrix account.~~
> * [ ] NEW: A simple binary allows to import messages from a ZBP account to a Matrix account.
> * [ ] The bridge detects messages that have already been imported to the a Matrix account and does not import them twice.
> * [ ] ~~The bridge is written in Rust/Python.~~
> * [ ] Reviewed by ~&quot;team::dev&quot;
> * [ ] Reviewed by ~&quot;team::architecture&quot;
>
> ## Notes/Hints/Ideas
>
> ![grafik](/uploads/fa90691b89a4326314f1b5584cf62c6f/grafik.png){width="900" height="294"}
>
> BundID's user-facing Frontend API can be accessed via `https://id.bund.de/api/...`
>
> **There is an intergration environment** at https://int.id.bund.de/de
>
> The API can be used after authenticating in a cookie-based session.
>
> Fetching ZBP messages from ZBP can be done as in the following script: [fetch-msg.py](/uploads/cd96b47223f03afe2933231d868a8b43/fetch-msg.py)
>
> The ZBP API documentation might be helpful (keep in mind this is the ZBP REST API and NOT the BundID Frontend API): https://nextcloud.fitko.de/f/2290808
>
> Timeline backfilling can only be done by Application Services using MSC2716, which is implemented in Synapse: https://github.com/matrix-org/synapse/pull/9247
>
> ## Who needs to be involved / informed
>
> - reviewer:
> - involved:
> - informed:

## #756 HTML-sanitize imported messages from BundID/ZBP

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/756
- Status: opened, angelegt 2026-08-17, zuletzt geändert 2026-11-13
- Labels: 3 Story Points, Refined, team::dev

> ## Problem
>
> We want a prototype of this mapping to gain experience.
>
> ## Goal
>
> Map ZBP message format to Neo message formats (remove ZBP-specific HTML tags).
>
> ## Out of scope
>
> - legal evaluation of implications of our mapping
>
> ## Prequisites
>
> - [ ] BundID test account with many messages
>
> ## ACs
>
> - [ ] The data from the test account is properly mapped to Neo events
> - [ ] An implementation of the mapping is done on the bridge
> - [ ] The sanitisation should follow the allowed tag list from the Matrix spec 
> - [ ] Markup / tag mapping documentation is available for the ZBP -> Neo contents
>
> ## Requirements
>
> - [ ] Provide requirement(s) UID(s) from requirements table.
> - [ ] [Neo spec](https://gitlab.opencode.de/fitko/matrix-g2x/solution-architecture/-/blob/develop/05_building_block_view.md#text) requires to only support Matrix HTML tags. The mapping is left open to the "migration concept".
>
> ## Notes / Resources
>
> - Sample data from AusweisApp default simulated identity can be used
>
> ## Who needs to be involved / informed
>
> - reviewers: 
> - involved: 
> - informed:

## #766 Concept for handling ZBP messages

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/766
- Status: opened, angelegt 2026-09-07, zuletzt geändert 2026-11-18
- Labels: Backlog, Needs pre-refinement, team::ux

> ## Problem
>
> > Messages may contain any tags as well as CSS (tables, buttons, links, animated backgrounds, styled CSS, etc.). This increases the risk that the styling of the messages will cause a break in the UI and that our usability standard cannot be maintained.
>
> ## Goal
>
> > A concept for how we can adapt incoming messages so that they fit our UI.
>
> ## ACs
>
> - [ ] We have a few sample messages (including some extreme examples) which we can use as a basis for developing our concept.
> - [ ] We have found a solution that enables us to remove or prevent customised styling from incoming messages.
> - [ ] Necessary follow-up issues relating to architecture and/or development exist.
>
> ## Requirements
>
> - [ ] Provide requirement(s) UID(s) from requirements table.
>
> ## Notes / Resources
>
> > Discussion from team days: https://nextcloud.fitko.de/apps/collectives/Neo-62/Breakout-Beautify-ZBP-messages-3314828
>
> - 
>
> ## Who needs to be involved / informed
>
> - reviewers: 
> - involved: 
> - informed:

## #107 Defining NDB-VN network connection to Matrix BE

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/107
- Status: opened, angelegt 2025-07-22, zuletzt geändert 2026-07-14
- Labels: Backlog, Prio::Medium, team::architecture, team::requirements

> **Problem:**
>
> - There is a network between the FIT-Connect SDK and the BundID BE, which is called NDB-VN.
> - The NDB-VN network communicates with the ZBP-Adapter (used to get messages through FIT-Connect into the ZPB BE).
> - The ZBP adapter de-crypts the message from the NDB-VN network.
> - The NDB-VN network should connect on to the Matrix Homeserver in the near future.
>
> **Goal:**
>
> Clarify the current  NDB-VN network components and interfaces used with BundID.
>
> **ACs:**
>
> - Define and describe NDB-VN and ZBP adapter relevant components.
> - Define components technology stack.
> - Describe interfaces and communication flow in a diagram.
> - Document the diagram in this ticket and in the following location: https://gitlab.opencode.de/fitko/matrix-g2x/solution-architecture
>
> **Notes/Resources:**
>
> - This point was discussed as part of the post box migration on the third workshop day. Source: https://nextcloud.fitko.de/apps/files/files/2053632?dir=/Messenger-Kommunikation/Architecture&openfile=true

## #812 Web: Adapt Design to Bundesportal

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/812
- Status: opened, angelegt 2026-10-05, zuletzt geändert 2026-11-19
- Labels: 8 Story Points, In Progress, Prio::Medium, Sprint 28, team::ux

> ## Problem
>
> > Our current design shows the BundID header (and footer) and colors, because we assumed we would integrate into the BundID portal.
>
> ## Goal
>
> > The web design is transferred to Bundesportal CI.
>
> ## ACs
>
> - [ ] Get Bundesportal styleguide (incl. files like logo, assets etc.)
> - [ ] Create color concept based on Bundesportal styleguide
> - [ ] (If needed) adapt colors to meet accessibility standards
> - [ ] Replace header
> - [ ] Replace footer
> - [ ] Get approval for publification
>
> ## Requirements
>
> - [ ] Provide requirement(s) UID(s) from requirements table.
>
> ## Notes / Resources
>
> > additional info that is helpful when working on the ticket
> > images, Figma, docs, etc
>
> ## Who needs to be involved / informed
>
> - reviewers: 
> - involved: 
> - informed:

## #105 [Epic] Notifications about new messages (Push, E-Mail)

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/105
- Status: opened, angelegt 2025-07-22, zuletzt geändert 2026-11-12
- Labels: EPIC, Prio::Low, team::architecture, team::dev, team::requirements, team::ux

> > [!note]
> >
> > This epic is reflected as a milestone in %&quot;Notifications about new messages (Push, E-Mail)&quot;
>
> ## Meta-Story
>
> As a citizen, I want to be notified by push notification and/or email when a new message from the public agency or status update arrives in my postbox, so that I'm promptly aware of any action required.
>
> ## Stakeholder value
>
> * Citizens learn promptly that something needs their action, without having to repeatedly check the app.
>
> ## Scope
>
> This epic covers deciding how citizens are informed about new messages and status updates including push and email notifications, and implementing the resulting concept across Neo clients and backend. It covers designing when each notification type (push vs. email) should be sent and how each should look, deploying the push gateway infrastructure needed to deliver mobile push notifications, implementing push notification support in the iOS app, and designing how email notifications can work together with Synapse's built-in email notification mechanism.
>
> ## Requirements
>
> * ZaPuK_19: \[The system\] enables \[real-time communication\] between two \[users\].
> * ZaPuK_84: The recipient of a message can be notified via third-party channels (inbox notification recipient) when a new message has arrived in their mailbox (e-mail or mobile push notification (app-based)).
> * ZaPuK_128: \[The system\] ensures that sensitive information, such as names and the content of a \[message\], is not transmitted in a form accessible to third parties (unencrypted) via push servers (from Google and Apple).
> * GA-Lotse_12: Push notifications would be nice to have.
>
> ## Completion Criteria
>
> - [ ] A decision is made and documented on how citizens are promptly informed about postbox messages (push-notifications), including whether email notifications are used.
> - [ ] A decision is made and documented on whether the Matrix backend sends all notifications (push, email) or not, and, if yes, an architecture decision for email notifications is made.
> - [ ] The UX concept defines when each notification type (push, email) is sent and what they look like.
> - [ ] The iOS app shows push notifications.
> - [ ] No sensitive information (names, message content) is ever transmitted unencrypted through third-party push infrastructure.
>
> ## Notes / resources
>
> * Which push notifications to show? -\> ~&quot;team::ux&quot;
> * How should push notifications look like -\> ~&quot;team::ux&quot;
> * Privacy considerations -\> ~&quot;team::requirements&quot;
> * Legal Requirements (OZG?) -\> ~&quot;team::requirements&quot;
> * How are push notifications generated (by Synapse?) and sent to device? -\> ~&quot;team::architecture&quot;
> * Mail-Notifications vs. Push-Notifications -\> ~&quot;team::requirements&quot; ~&quot;team::ux&quot;
> * Technical foundations to be able to show push notifications -\> ~&quot;team::dev&quot;
>
> Issues
>
> * https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/issues/229+
> * https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/issues/465+
> * https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/issues/464+
> * https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/issues/466+
>
> ## Who needs to be involved / informed
>
> - involved:
> - informed:

## #229 UX Concept | Mail and Mobile Push Notifications

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/229
- Status: opened, angelegt 2025-09-09, zuletzt geändert 2026-11-18
- Labels: 3 Story Points, Prio::Low, Ready for Sprint, Sprint 28, team::ux

> ## Description:
>
> As a user, I want to do get notified about new messages and other activity so that I can return to the app/web client to get more information / interact with the public agency.
>
> ## Acceptance Criteria:
>
> - [ ] The concept answers the question when to send which type of notification (push, mail).
> - [ ] We have the content that is in the notification
> - [ ] we have the content that is in the mail
> - [ ] speak to arc about possible information & SOP for secure content (e.g. links)
> - [ ] Follow up tickets are created
>
> ## Requirements:
>
> * [ ] ZaPuK_19 : \[The system\] enables \[real-time communication\] between two \[users\].
> * [ ] ZaPuK_84 : The recipient of a message can be notified via third-party channels (inbox notification recipient) when a new message has arrived in their mailbox (e-mail or mobile push notification (app-based)).
>
> ## Documents & Resources:
>
> - Insights from user test in October about necessary information
>
> ## Note
>
> * What will we do if there is several messages sent? Can we send them just one mail/notification? 
>
> ## Who needs to be involved / informed:
>
> - reviewers:
> - involved:
> - informed:

## #464 iOS App: Support Push Notifications

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/464
- Status: opened, angelegt 2026-02-11, zuletzt geändert 2026-10-27
- Labels: 5 Story Points, Prio::Low, Refined, team::dev, topic::ios

> ## Description:
>
> As a citizen, I want to be notified via a Push Notification when there are new activities or updates to my applications.
>
> ## Acceptance Criteria:
>
> - [ ] Updates to a case trigger push notifications
> - [ ] A push notification is shown on the user's device with minimal information (The app's icon, name and a generic message such as "There are updates to your case")
> - [ ] Tapping on the notification opens the app on the Case Overview screen
>
> ## Requirements
>
> * [ ] ZaPuK_19 : \[The system\] enables \[real-time communication\] between two \[users\].
> * [ ] ZaPuK_84 : The recipient of a message can be notified via third-party channels (inbox notification recipient) when a new message has arrived in their mailbox (e-mail or mobile push notification (app-based)).
> * [ ] ZaPuK_128 : \[The system\] ensures that sensitive information, such as names and the content of a \[message\], is not transmitted in a form accessible to third parties (unencrypted) via push servers (from Google and Apple).
> * [ ] GA-Lotse_12 : Push notifications would be nice to have.
>
> **Notes:**
>
> > Any link to any other task, prior knowledge that might help solving this, or done something similar before?
>
> **Risks:**
>
> > Any pitfalls, that we already know of? How to avoid them?
>
> ## Who needs to be involved / informed:
>
> - reviewers:
> - involved:
> - informed:

## #820 Define Storage Period for Messages

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/820
- Status: opened, angelegt 2026-10-29, zuletzt geändert 2026-11-12
- Labels: Backlog, Prio::Medium, team::requirements

> ## Problem
>
> We are incorporating different solutions, so we will send and receive messages from different existing solutions. All these solutions have their own requirements for their storage period. Right now, we do not have a solution on how to handle all these different requirements
>
> ## Goal
>
> We want to have a well documented and well-discussed decision on how to handle storage time. This is to be defined as part of the solution architecture.
>
> ## ACs
>
> - [ ] 
>   * [ ] The sources we need to consider are defined
>   * [ ] We have a requirement for the max number of attachments
>   * [ ] here is a documentation available for the solution arc
>   * [ ] The requirements are cross checked with arc team
>   * [ ] The requirement is pushed in Git.
>
> ## Requirements
>
> - [ ] Provide requirement(s) UID(s) from requirements table.
>
> ## Notes / Resources
>
> > additional info that is helpful when working on the ticket images, Figma, docs, etc
>
> - 
>
> ## Who needs to be involved / informed
>
> - reviewers:
> - involved:
> - informed:

## #799 Define Quality Requirements regarding Message and Attachments Size and Formats

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/799
- Status: opened, angelegt 2026-10-01, zuletzt geändert 2026-10-05
- Labels: Backlog, Prio::Medium, team::requirements

> ## Problem
>
> We are incorporating different solutions, so we will send and receive messages from different existing solutions. All these solutions have their own requirements for their attachments and messages. Right now, we do not have a solution on how to handle all these different requirements
>
> ## Goal
>
> We want to have a well documented and well-discussed decision on how to handle attachment-sizes, format, number and general message size. This is to be defined as part of the solution architecture.
>
> ## ACs
>
> - [ ] The sources we need to consider are defined
> - [ ] We have a requirement for the max number of attachments
> - [ ] We have a requirement for the max size per attachment
> - [ ] We have a requirement for the max total size of message
> - [ ] We have a requirement for the allowed filetypes
> - [ ] We have a requirement for the file name restrictions
> - [ ] We have a clear understanding of requirements for MJP written form
> - [ ] We have a requirement for sending electronic evidence via MJP
> - [ ] There is a documentation available for the solution arc
> - [ ] The requirements are cross checked with arc team
> - [ ] The requirement is pushed in Git.
>
> ## Requirements
>
> * [ ] ZaPuK_18: \[The system\] enables the legally binding and confidential transmission of \[messages\] and \[attachments\].
> * [ ] ZaPuK_35: From a technical perspective, \[the system\] is capable of sending \[messages\] and \[attachments\] of any size.
>
> ## Notes / Resources
>
> * different requirements for sending and receiving
> * depends on recepient (e.g. specific limits when sending to EGVP)
> * Bundesportal already seems to have requirements for that. We could also ask them what their requirements are 
>
> ## Who needs to be involved / informed
>
> - reviewers:
> - involved:
> - informed:

## #303 Clarify legal requirements regarding Zustellfiktion and legal evidence of transmission

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/303
- Status: opened, angelegt 2025-11-04, zuletzt geändert 2026-11-18
- Labels: 8 Story Points, Prio::0, Ready for Review, Sprint 24, Sprint 25, Sprint 26, Sprint 27, Sprint 28, team::requirements

> ## Problem
>
> * ZaPuK requests that the system logs sending and receiving messages, and informs the sender and recipients of this.
> * However, logging from messages that are sent and received generates a direct conflict with user's unlinkability. The implementation of E2EE through Matrix also brings with it the challenge of guaranteeing the provision of proof of delivery when required.
> * Bereitstellungsbestätigung: ZaPuK-ADR-0018 states that the delivery confirmation (Bereitstellungsbestätigung) is to be implemented as a special unencrypted Matrix event, signed by the server to prove that the receipt arrived within the recipient’s sphere of influence (Machtbereich des Empfängers).
> * Abrufbestätigung und Lesebestätigung: ZaPuK-ADR-1118 states that a message has been acknowledged by the recipient through a retrieval confirmation (Abrufbestätigung) and/or through a read confirmation (Lesebestätigung) which should be implemented as encrypted room events, since the homeserver cannot determine whether or when a retrieval or read confirmation was sent.
>
> ## Goal
>
> Define how Neo can provide legally reliable, verifiable evidence of message transmission, delivery, and acknowledgement that meets ZaPuK's notification requirements (Bereitstellungsbestätigung, Abrufbestätigung, and Lesebestätigung) without jeopardizing user unlinkability or Matrix E2EE.
>
> ## ACs
>
> - [ ] A list of requirements about Constructive Notices (Zustellfiktion) and legal evidence of Delivery, Retrival and Read Receipts is defined.
> - [ ] The requirements include a reference to law if they are baked by law.
> - [ ] The requirements include a reference to the ZaPuK legal opinion if they are backed by the legal opinion.
> - [ ] Requirements that are not baked by law or the legal opinion explicitely state this fact.
> - [ ] The list of requirements is exhaustive.
> - [ ] The list of requirements is approved by ZaPuK team.
> - [ ] Reviewed by ~&quot;team::architecture&quot;.
>
> ## Requirements
>
> - [ ] ZaPuK_85 : \[The system\] logs the (legally relevant) time of delivery of a \[message\] and informs the \[sender\] and \[recipients\] of this.
> - [ ] ZaPuK_86 : \[The system\] enables legally compliant and reliable delivery of \[messages\] to \[public authorities\] when required, which cannot be disputed by any communication partner.
> - [ ] ZaPuK_87 : \[The system\] automatically sends \[sender\] and \[recipient\] a \[delivery confirmation\] when a \[message\] is successfully sent to a \[recipient\].
> - [ ] ZaPuK_88 : \[The system\] automatically sends \[sender\] and \[recipient\] a \[confirmation of retrieval\] in the event of a successful removal of a \[message\] from the mailbox by the \[recipient\]
> - [ ] ZaPuK_90 : \[The system\] ensures that \[messages\] are not lost even if transmission attempts fail.
> - [ ] ZaPuK_145 : \[The system\] identifies incoming and outgoing \[messages\] in the \[mailbox\].
> - [ ] LB_52 : The architecture must include mechanisms for maintaining and expanding the system (mechanisms for troubleshooting, monitoring logs, and integrating new functionalities).
>
> ## Notes / Resources
>
> * Requirements are being documented here as this ticket's comments underneath.
> * ZaPuK-ADRs: https://nextcloud.fitko.de/f/2730126
> * Legal basis:
>
> 1. Zustellfiktion ('fiction of service') closest definition ([§ 173 Abs. 4 S. 4–5 ZPO](https://www.gesetze-im-internet.de/zpo/__173.html)) :
>
> > An electronic document is deemed to have been delivered on the fourth day following the date of _arrival (Eingangs)_ indicated on the automated proof of _receipt (Eingangsbestätigung)_ in the electronic mailbox opened by the recipient.
>
> 2. Arrival (Empfangstheorie) definition ([§ 130 Abs. 1 BGB](https://www.gesetze-im-internet.de/bgb/__130.html)) -\> Wirksamwerden der Willenserklärung gegenüber Abwesenden (Effective Date of the Declaration of Intent):
>
> > Eine Willenserklärung, die einem anderen gegenüber abzugeben ist, wird, wenn sie in dessen Abwesenheit abgegeben wird, in dem Zeitpunkt wirksam, in welchem sie ihm zugeht. Sie wird nicht wirksam, wenn dem anderen vorher oder gleichzeitig ein Widerruf zugeht.
>
> 3. Eingangsbestätigung (proof of receipt) definition ([§ 130a Abs. 5 ZPO)](https://www.gesetze-im-internet.de/zpo/__130a.html) :
>
> > An electronic document is considered to have been received as soon as it is stored on the facility designated (Einrichtung des Gerichts) as a receiving system.
>
> 4. Facility designated (Einrichtung des Gerichts) as a receiving system definition : no statutory definition. § 130a Abs. 5 S. 1 just uses the phrase "auf der für den Empfang bestimmten Einrichtung des Gerichts gespeichert ist" without defining it. The Best approach here is to point out to the Zugang/Empfangstheorie doctrine under § 130 Abs. 1 BGB, namely a message enters the recipient's sphere once it reaches a facility the recipient keeps ready to receive (Machtbereich). In the ERV system, for example, the designated receiving facility is the OSCI intermediate.
> 5. Laws defining guidelines on how to build electronic documents to support an electronic acknowledgment of receipt (elektronisches Empfangsbekenntnis):
>    1. Technical standards for delivering a notice : [§ 5 Abs. 1 ERVV](https://www.gesetze-im-internet.de/ervv/__5.html)
>    2. Requirements on electronic documents: [§ 2 Abs. 3 ERVV](https://www.gesetze-im-internet.de/ervv/__2.html)
> 6. Notification of adm. act (Bekanntgabe des Verwaltungsaktes) [§ 9(1) OZG ](https://www.gesetze-im-internet.de/ozg/__9.html): In cases of doubt, the authorities must prove the timing and method in which a Bescheid was delivered.
> 7. Abrufbestätigung has no legal implication and don't have any legal effect on Zustellfiktion itself, particularly in the case of private individuals who aren't required to deliver a prove of service to the recipient (elektronisches Empfangsbekenntnis, or eEB - § 173 Abs. 3 ZPO)**.** However, Abrufbestätigung can be seen as a partial help mechanism (Hilfsmittel) for Zustellfiktion when an Empfangsbekenntnis is missing, and as supporting evidence that the actual notice/ document arrived, when required ([§ 189 ZPO](https://www.gesetze-im-internet.de/zpo/__189.html)) :
>
>    > If it cannot be proven that a document was served in the proper form ..., it shall be deemed to have been served at the time the document was actually received by the person...
> 8. Lesebestätigung, unlike the Abrufbestätigung, is meant to serve as the elektronisches Empfangsbekenntnis under [§ 173 Abs. 3 ZPO](https://www.gesetze-im-internet.de/zpo/__173.html), [§ 175 ZPO](https://www.gesetze-im-internet.de/zpo/__175.html) and [§ 5 VwZG](https://www.gesetze-im-internet.de/vwzg_2005/__5.html). Neither confirmation affects the Zustellfiktion, because that is the Bereitstellungsbestätigung's role. The Lesebestätigung follows the same principle as the eEB, but it does not yet meet the legal requirements to count as one. The eEB is only legally required as proof of electronic delivery to the parties listed in [§ 173 Abs. 2](https://www.gesetze-im-internet.de/zpo/__173.html), not citizens, when requested transmitted to the court.
>
> * Bereitstellungsbestätigung: [ADR-0018](https://nextcloud.fitko.de/apps/files/files/2727578?dir=/01_IT-PLR/01_Gremien/itPLR-21-017_FIT-AB/00_ZaPuK/1%20-%20Bereich%20FIT-AG/Architekturentscheidungen-ADR&editing=false&openfile=true) , section 3.
> * Abrufbestätigung und Lesebestätigung: [ADR-1118](https://nextcloud.fitko.de/apps/files/files/2727563?dir=/01_IT-PLR/01_Gremien/itPLR-21-017_FIT-AB/00_ZaPuK/1%20-%20Bereich%20FIT-AG/Architekturentscheidungen-ADR&editing=false&openfile=true), Architekturentscheidung.
> * The ARDs are to be treated as drafts, not specs. The technical implementation suggested by the ADRs are open to de discussed and improved if needed.
> * Relevant sources for further related functionalities: ADRs-1119 and 1120.
> * Related epic: Read receipt, transmission receipt, etc.? -\> see https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/582
> * QES-signed Lesebestätigung as an elektronisches Empfangsbekenntnis (§ 175 ZPO / § 173 Abs. 3 ZPO), carries legal, fristauslösende effect but belongs to the authority recipient regime and depends on QES/EUDI-Wallet availability.
>
> ## Who needs to be involved / informed
>
> - reviewers:
> - involved:
> - informed:

## #554 [Epic] Login via EUDI-Wallet

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/554
- Status: opened, angelegt 2026-03-20, zuletzt geändert 2026-09-30
- Labels: EPIC, Needs pre-refinement, Out of scope Phase IV, team::dev, team::ux

> > [!NOTE]
> > This epic is reflected as a milestone in https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/milestones/63
>
> ## Meta-Story
>
> As a private individual, I want to login using the EUDI-Wallet, so that I have an alternative login mechanism to using my eID card / or my BundID account.
>
> ## Scope
>
> > Which part of the product does this apply to? Where are the limits, what is out of scope?
>
> ## Issues
>
> > List issue here that belong to this Epic.
>
> - ...
>
> ## Completion
>
> > When is this feature complete? How can we verify that the goal was reached?
>
> ## Notes / resources
>
> > links to mockups, Figma, pictures, documentation
>
> ## Who needs to be involved / informed
>
> - involved:
> - informed:

## #553 [Epic] Issue Attestations to EUDI-Wallet

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/553
- Status: opened, angelegt 2026-03-20, zuletzt geändert 2026-09-30
- Labels: EPIC, Needs pre-refinement, Out of scope Phase IV, team::architecture, team::dev, team::ux

> ## Meta-Story
>
> > [!NOTE]
> > This epic is reflected as a milestone in https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/milestones/62
>
> As a private individual who received an official decision from a government agency, I want to import an attestation into my EUDI-Wallet, so that I can present that attestation to a 3rd party using my EUDI-Wallet.
>
> ## Scope
>
> > Which part of the product does this apply to? Where are the limits, what is out of scope?
>
> tbd: PubEAA and/or QEAA
>
> ## Issues
>
> > List issue here that belong to this Epic.
>
> - ...
>
> ## Completion
>
> > When is this feature complete? How can we verify that the goal was reached?
>
> ## Notes / resources
>
> - BundID will go live with [Dresen-Pass](https://www.dresden.de/de/rathaus/dienstleistungen/dresdenpass_d115.php) and [Ehrenamtskarte](https://www.dresden.de/de/rathaus/dienstleistungen/ehrenamtskarte.php) until 2027-01
>
> ## Who needs to be involved / informed
>
> - involved:
> - informed:

## #513 Automatically redirect for BundID IdP Auth

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/513
- Status: opened, angelegt 2026-03-03, zuletzt geändert 2026-10-13
- Labels: 5 Story Points, In Progress, Prio::High, Sprint 19, Sprint 20, Sprint 21, Sprint 22, Sprint 23, Sprint 25, Sprint 26, team::infra-and-ops

> ## Problem
>
> We don't want users to notice that they're being redirected through MAS and Keycloak when loggin in in the app
>
> ## Goal
>
> We want to provide **different native buttons** in our clients that choose a different OAuth upstream provider:
> - A **developer mode login** using manual MAS accounts (possibly through an existing session in the browser context)
> - A **BundID login** redirecting to the id.bund.de web interface with minimal distrubances and or loading indicators
> - (Mobile only) An "**eID login**" using BundID under the hood, using different Upstream in Keycloak that is a hosted site by us and redirects back to our app via a scheme handler.
>
> As such we need to be able to pass along a value to Keycloak via MAS.
>
> ## ACs
>
> - [ ] A first patch against MAS exists for passing along a custom parameter (see #579)
> - [ ] Configure Keycloak to support taking the parameter passed by MAS
> - [ ] Configure Keycloak to have a second, identical BundID upstream (the URL of this will change in the future, the SAML response will be the same as for vanilla BundID) with the goal to test mapping of user IDs.
> - [ ] When specifying an upstream for an OAuth flow, MAS and Keycloak will not show up in the regular login flow
> - [ ] Navigating to the bare MAS still allows loggin in with manual test accounts.
> - [ ] Its possible to login into Element Web on ess1 via MAS and via int.id.bund.de.
> - [ ] Reviewed by @networkException
>
> ## Requirements
>
> none
>
> ## Notes / Resources
>
> - There exists prior art with such state in MAS for an implementation of [MSC4198](https://github.com/matrix-org/matrix-spec-proposals/pull/4198)
> - Keycloak supports passing [`kc_idp_hint`](https://www.keycloak.org/docs/latest/server_admin/#_client_suggested_idp) to decide on an upstream
> - See also [this thread](https://matrix.to/#/!ethZaECJGUnNhVlNKA:element.io/$9vMYJ0ieujLOCSeTJdkk7kizwaQ171bfH1wZRhdhtKk?via=nordeck.io&via=matrix.org) in [#infrastructure.neo:fitko.dev](https://matrix.to/#/#infrastructure.neo:fitko.dev) for options being discussed
>
> ## Who needs to be involved / informed
>
> - reviewers: @networkException 
> - involved: @justjanne 
> - informed: @mgcm

## #829 Align Neo glossary and texts with the ZaPuK glossary

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/829
- Status: opened, angelegt 2026-11-11, zuletzt geändert 2026-11-18
- Labels: Ready for Review, Sprint 28, zapuk

> ## Problem
>
> > The ZaPuK glossary now includes an English term for every entry: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/main/Glossar.md
> >
> > NEO's glossary (12_glossary.md) and other texts use different English terms for the same concepts. E.g. "agency" is used for Behörde (authority), "delivery receipt" for Bereitstellungsbestätigung (delivery confirmation), or "constructive notice" for Bekanntgabefiktion (deemed notification). 
> >
> > Some terms have different legal meanings in English. Using different terms for the same concept leads to misunderstandings between Neo and ZaPuK.
>
> ## Goal
>
> > Neo uses the same terms as ZaPuK. The ZaPuK glossary is the single source of truth. The Neo glossary is no longer maintained. 12_glossary.md only refers to the ZaPuK glossary. Neo terms missing in ZaPuK and the English definitions of Neo are moved to the ZaPuK glossary.
>
> ## ACs
>
> - [x] Neo terms missing in the ZaPuK glossary are added there with German and English definition
> - [x] English definitions of Neo are added to the matching ZaPuK entries, corrected where they contradict the ZaPuK glossary, without references to laws
> - [x] 12_glossary.md refers to the ZaPuK glossary and contains no entries of its own
> - [x] Concept 303, the arc42 chapters and the NEO-ADRs use the glossary terms (see list below)
>
> ## Requirements
>
> * ZaPuK glossary
>
> ## Notes / Resources
>
> > - Builds on MR !56.
> > - Main changes (old -\> new, German):
> >   - (government) agency -\> authority, competent authority (Behörde, zuständige Behörde), where the Behörde of a case is meant
> >   - (government) agency -\> public body (öffentliche Stelle), for the public side in general
> >   - IT system of the competent agency -\> back-office system (Fachverfahren)
> >   - delivery receipt -\> delivery confirmation (Bereitstellungsbestätigung)
> >   - confirmation of retrieval, retrieval receipt -\> retrieval confirmation (Abrufbestätigung)
> >   - read receipt -\> read confirmation (Lesebestätigung)
> >   - deemed service -\> deemed service (Zustellfiktion) and deemed notification (Bekanntgabefiktion)
> >   - constructive notice -\> deemed notification (Bekanntgabefiktion)
> >   - provisioning -\> making available for retrieval (Bereitstellung zum Abruf)
> >   - notification -\> new message alert (Eingangsbenachrichtigung), where the alert is meant
> >   - case -\> case (Vorgang) and administrative procedure (Verwaltungsverfahren)
>
> ## Who needs to be involved / informed
>
> - reviewers: @holz
> - involved: @gerbauld
> - informed: @kim

## #832 Review v1.1 concept

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/832
- Status: opened, angelegt 2026-11-13, zuletzt geändert 2026-11-18
- Labels: In Progress, Sprint 28, zapuk

> Preparing publication: concept, ADRs, glossary and legal opinion references on `architecture-v1.1` were aligned with the actual ADRs. 
>
> ## Goal
>
> All content changes are reviewed, corrected and approved. The branch is ready to publish.
>
> ## ACs
>
> - [ ] Concept, ADR, other docs reviewed, acutalized, corrections applied
> - [ ] Changes committed and pushed
>
> ## Reviewed by
>
> @SilvanVerhoeven 
>
> ## Requirements
>
> n/a

## #831 Slide decks ZaPuK and Matrix

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/831
- Status: opened, angelegt 2026-11-13, zuletzt geändert 2026-11-18
- Labels: In Progress, Sprint 28, zapuk

> ## Problem
>
> > Need slide decks to explain why ZaPuK, why Matrix, the difference between a central and a federated mailbox, and how identities work. (for face-to-face BMDS-meetings, for Federführer in ZaPuK)
>
> ## Goal
>
> > - Deck 1: ZaPuK and Matrix (30 slides)
> > - Deck 2: identities, addressing and unlinkability
> > - Deck 3: ZBP and the mailboxes in Mein Unternehmenskonto
>
> ## ACs
>
> - [ ] Build deck 1
> - [ ] Build deck 2
> - [ ] Build deck 3
>
> ## Notes / Resources
>
> > 
>
> ## Who needs to be involved / informed
>
> - reviewers:
> - involved:
> - informed:

## #159 iOS App: Using the iOS app to login into web app ("device switch")

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/159
- Status: opened, angelegt 2025-08-08, zuletzt geändert 2026-11-17
- Labels: Backlog, Needs pre-refinement, Prio::Low, blocked, team::dev

> ~blocked by #771 
>
> ## Problem
>
> Login using eID via Web browser on devices without an NFC reader is pain.
>
> ## Goal
>
> When logging in into the web app, we want to use the iOS app as an eID client.
>
> This issue describes the **iOS-side implementation** part of loggin into the web client via eID via the iOS app. See #226 for the other side of the implementation in the web client.
>
> ## ACs
>
> - [ ] UI from figma is fullfilled
> - [ ] ...
>
> ## Pre-requisites
>
> - Enable e2ee cross-signing set up in the iOS App ([ref](https://github.com/matrix-org/matrix-spec-proposals/blob/element-hq/oidc-qr-login/proposals/4108-oidc-qr-login.md#existing-device-scanning-the-qr-code))
>
> ## Requirements:
>
> * [ ] ZaPuK_142 : Access to \[Messages\] can be gained via multiple devices, which offer the same view of the communication content.
> * [ ] LB_27 : The authentication methods support integration and interoperability with existing systems and a seamless user experience.
> * [ ] OZG_27 : Electronic identification can be done by a one-time request for identity data.
>
> ## Notes/ resources
>
> - ZaPuK
>   - Device Switch for eID and future privacy requirements for the Wallet were considered in [ADR-0014: Authentizität: Identität von Privatpersonen](https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/main/Zielarchitektur/Architekturentscheidungen/ADR-0014-Authentizit%C3%A4t-Identit%C3%A4t-von-Privatpersonen.md)
> - FIDO
>   - https://www.goodsignin.com/blog/what-does-cable-have-to-do-with-passkeys (this is a good primer on what DigitalService has done, specifically with respects to phishing resistance / proximity)
> - DigitalService Webauthn experiments
>   - [Video](https://github.com/digitalservicebund/useid-documentation?tab=readme-ov-file#5-exploration-ger%C3%A4twechsel-mit-webauthn)
>   - Problem Statement for Multi-Device / Device Switch with eID (best to read this before the research document, the FIDO caBLE Blog post is also a good primer): https://github.com/digitalservicebund/useid-architecture/blob/main/research/device-switch/problem-statement.md
>   - https://github.com/digitalservicebund/useid-architecture/blob/main/research%2Fdevice-switch%2Fevaluation-of-approaches.md#webauthn
> - W3C
>   - Currently quite oriented around Wallet concepts. The main difference to eID is that the actual presenting of the credential is meant to be [non interactive](https://en.wikipedia.org/wiki/Non-interactive_zero-knowledge_proof) in the wallet use case.
>   - https://www.w3.org/TR/digital-credentials/
>   - W3C-Explainer: [Threat Model - Presenting Credentials on the Web](https://docs.google.com/document/d/1Ppaz_EnhzHqPOz5UusRJvbSunh-RXPWgJ3Np_TM2EE0/edit?tab=t.0)
> - Google
>   - https://blog.google/technology/safety-security/opening-up-zero-knowledge-proof-technology-to-promote-privacy-in-age-assurance/
>
> ## Who needs to be involved / informed
>
> - reviewers:
> - involved:
> - informed:

## #225 iOS App: Login via eID

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/225
- Status: opened, angelegt 2025-09-09, zuletzt geändert 2026-11-18
- Labels: 5 Story Points, Presented in Review Meeting, Prio::High, Ready for Review, Sprint 22, Sprint 24, Sprint 25, team::dev, topic::ios

> ## Problem
>
> While the iOS app already integrates the AusweiApp SDK, there is currently no UI for logging in using an eID card.
>
> ## Goal
>
> A minimal UI allows to login into BundID using an eID card within the app.
>
> In this issue, we want to focus on functionality. Polishing the UI and improving user experience is out of scope of this issue. Still, we want to hide as much unneccessary steps in the user interface as possible.
>
> ## ACs
>
> - [x] The BundID login is initiated from within the app.
> - [x] Users are able to login using their eID card (trust level _high_).
> - [x] When the entered PIN of the eID card is incorrect, the app allows to re-enter another PIN.
> - [x] The app detects if the eID card is not set-up yet or locked and shows a warning (implementing the setup/restore process is out of scope of this issue).
>
> ## Requirements
>
> * [x] ZaPuK_107: \[Users\] can be authenticated up to the “high” trust level (according to BSI TR-03107-1)
> * [ ] ~~ZaPuK_108: \[The system\] only allows access to \[messages\] by the \[recipient\] if the \[recipient\] has been authenticated with the trust level defined by the \[sender\] for this \[message\].~~
> * [x] LB_25: User authentication should be enabled via eID and BundID.
> * [x] LB_28: The basic functionality for authentication using an identity card (eID) is provided directly in the (iOS) app to be developed.
> * [ ] ~~LB_60: End-to-end authentication: the identity of the communication partners is verified reliably at all times.~~
> * [ ] ~~LB_67: Protection of sensitive data, securing interfaces, management of access rights.~~
> * [x] LB_177: Authentication: Authentication with the Matrix server in the app is performed directly using eID via the integration of the AusweisApp SDK.
> * [ ] ~~OZG_1: The identification and authentication of users should be carried out via a zentrales Bürgerkonto in order to access Verwaltungsleistungen.~~
> * [ ] ~~OZG_18: In order to verify the identity of the user(s) of a citizen account, the data from the identity card with eID function, notified European eID means or any data from other recognized means of identification, and the mailbox reference may be processed (if necessary).~~
>
> ## Notes/ resources
>
> See BundesIdent results.
>
> ## Who needs to be involved / informed
>
> - reviewers:
> - involved:
> - informed:

## #186 eID-based Device Verification: Describe problem domain for project-external people

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/186
- Status: opened, angelegt 2025-08-19, zuletzt geändert 2026-11-18
- Labels: 5 Story Points, Prio::High, Ready for Sprint, Sprint 26, Sprint 28, team::architecture

> (preparatory work for #630)
>
> ## Problem
>
> Before sending sensitive information to private individuals, competent agencies would like to verify the end-to-end-encrypted channel between them and the private individual ("device verification"). Private individuals need to verify their devices using their BundID credentials (trust level: normal and substantial) or via eID (trust level high).
>
> This issue is about **eID-based device verification (trust level high)**.
>
> ## Goal
>
> End result (see #630): Define how eID-based device verification will allow competent agencies to verify the owner of a private individuals Matrix device.
>
> **In this issue, we focus on describing the interacting components and the general problem definition of achieving cryptographic binding of the eID authentication to the Olm/MegOlm channel solely. We want to create a document that we can send to project-external people**
>
> ## ACs
>
> - [ ] A brief markdown document describes the high-level goal that we want to achieve.
> - [ ] The document describes the interaction components.
> - [ ] The document explains the usage of Olm/Megolm in an application-agnostic way (crypto terminology, not Matrix terminology)
> - [ ] The document describes what we want to achieve on the cryptographic level.
> - [ ] The document is understood by project-external people without requiring project-specific knowledge.
>
> ## Requirements:
>
> - [ ] ZaPuK_111: \[The system\] ensures the authenticity of a \[message\] at the trust level at which the \[sender\] was authenticated.
> - [ ] LB_30: The identity of the communication partner (i.e., the competent authority) and the authenticity of the established communication channel must be verified beyond doubt.
> - [ ] OZG_3: Proof of identity for users of administrative services with trust level "high" is provided by means of an electronic identity card, an electronic residence permit, or an eID card for citizens of the EU and the EEA with an activated online ID.
> - [ ] OZG_18: In order to verify the identity of the user(s) of a citizen account, the data from the identity card with eID function, notified European eID means or any data from recognized means of identification and the mailbox reference may be processed (if necessary).
> - [ ] ~~ZaPuK_109: The recipient of a message can verify the level of trust at which the sender has been authenticated.~~
> - [ ] ~~ZaPuK_110: Message communication can take place at different levels of trust, for which appropriately approved \[authentication means\] must be used.~~
> - [ ] ~~LB_26: Only verified users have access to the protected communication infrastructure, ensuring the integrity of the system.~~
> - [ ] ~~LB_37: eIDAS Level of Assurance (LoA) is required.~~
> - [ ] ~~LB_36: Automated verification of the identity of the private individual communicating via eID/BundID login is possible.~~
> - [ ] ~~LB_60: End-to-end authentication: the identity of the communication partners is verified reliably at all times.~~
> - [ ] ~~LB_67: Protection of sensitive data, securing interfaces, management of access rights.~~
> - [ ] ~~OZG_1: The identification and authentication of users should be carried out via a zentrales Bürgerkonto in order to access Verwaltungsleistungen.~~
>
> ## Notes / Resources
>
> ## Who needs to be involved / informed
>
> - reviewers:
> - involved:
> - informed:

## #692 Research: Status Quo in Matrix on Search on Encrypted Messages

- URL: https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/692
- Status: opened, angelegt 2026-07-10, zuletzt geändert 2026-10-29
- Labels: 3 Story Points, Prio::0, Ready for Review, Sprint 23, Sprint 24, Sprint 25, matrix-upstream, team::architecture

> ## Problem
>
> We don't know details about past efforts to realize search on encrypted messages - not only in native clients but also in web-based szenarios
>
> ## Goal
>
> We assume that this feature should be implemented in matrix-rust-sdk. -\> Validate this assumption!
>
> - What is the current status of this feature?
> - Are there MSCs?
> - Will this require additional MSCs?
> - Are there (prototype) implementations?
> - How could a path towards implementing search on encrypted messages in matrix-rust-sdk look like (incl. first estimation of effort/costs)?
>
> ## ACs
>
> - [ ] Answer the questions raised in the "goal" section as a comment in this issue.
> - [ ] The solution allows search in iOS, Web and the FIT-Connect SDK (and also Android once we support it).
> - [ ] Reviewed by ~&quot;team::architecture&quot;
>
> ## Requirements
>
> - [ ] Provide requirement(s) UID(s) from requirements table.
>
> ## Notes / Resources
>
> > additional info that is helpful when working on the ticket images, Figma, docs, etc
>
> - https://github.com/matrix-org/matrix-rust-sdk/tree/66ba8640c367490550419d780d2f439e12c4a00c/crates/matrix-sdk-search -\> matrix-rust-sdk seems to have an experimental implementation. Unsure about wasm support. It seems to be possible to use in the ffi bindings though. So it may work on iOS? -\> To be confirmed
> - https://github.com/matrix-org/matrix-rust-sdk/blob/66ba8640c367490550419d780d2f439e12c4a00c/bindings/matrix-sdk-ffi/Cargo.toml#L28
> - matrix-rust-sdk: [Meta: Full text search support](https://github.com/matrix-org/matrix-rust-sdk/issues/5350)
> - matrix-rust-sdk: [Message search: next steps](https://github.com/matrix-org/matrix-rust-sdk/issues/7353)
> - Element-Meta: [[Story] Search for messages](https://github.com/matrix-org/matrix-rust-sdk/issues/5350)
>
> ## Who needs to be involved / informed
>
> - reviewers:
> - involved:
> - informed:

