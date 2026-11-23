# Element (Blog): Matrix-based ZaPuK confirmed as a core component within Germany’s Deutschland-Stack

Quelle: https://element.io/blog/matrix-based-zapuk-confirmed-as-a-core-component-within-germanys-deutschland-stack/
Datum Dokument: 2026-06-30
Abgerufen: 2026-11-19

Sekundärquelle mit Eigeninteresse: Element ist Mitglied des Neo-Auftragnehmerkonsortiums (adesso, Element, Nordeck). Nur Artikeltext.

---

# Matrix-based ZaPuK confirmed as a core component within Germany’s Deutschland-Stack

On 17 June Germany’s federal government and 16 Länder (states), led by The IT Planning Council, confirmed that ZaPuK will be a cornerstone of Germany’s Deutschland-Stack .

The Deutschland-Stack (D-Stack) is Germany’s national sovereign technology platform designed to serve as a uniform, cross-governmental IT architecture. Managed by the Federal Ministry for Digital Affairs and State Modernisation (BMDS) , it aims to consolidate Germany's highly fragmented federal, state and municipal IT systems into a unified digital operating system by 2028. It does this through the use of a shared ‘construction kit’ of reusable baseline services, including secure infrastructure and interoperable open source components.

As the Matrix open standard is already embedded in ZaPuK, Matrix is now also a part of Deutschland-Stack and will help ensure sovereign and interoperable communications between Germany’s government agencies and the citizens and companies they serve.

Germany’s ZaPuK (Zielarchitektur Postfach- und Kommunikationslösungen) initiative, led by FITKO , Saxony-Anhalt and the Free Hanseatic City of Hamburg, creates a consistent technical architecture for mailbox and communications components. The ZaPuK project is Germany's answer to a question that has troubled public administrations for decades; how do citizens, companies, and government agencies communicate with each other reliably and securely without a proliferating ecosystem of incompatible inboxes?

It’s precisely the type of challenge that the decentralised Matrix open standard was designed to address. Matrix obliterates vendor-led self interest because it's an independent open standard. Matrix is decentralised, which separates individual deployments (servers), applications (frontends) and the network. As a result, Matrix-based components and solutions are all interoperable.

Matrix is now part of a defined component of the Deutschland-Stack, with transition planning underway and operational deployment targeted for 2028. Over time various Matrix-based solutions will replace a patchwork of incompatible mailbox solutions including, as reported in Heise , De-Mail, Mein Justizpostfach, Zentrales Bürgerpostfach, Elster-Postfach and EGVP.

These types of portals and apps typically enable citizens and companies to interact with state-level government and government agencies to arrange everyday requirements from planning permissions to parking permits.

### See FITKO’s Dominik Braun presenting on Matrix-based architecture to serve Germany’s administrative communication

Matrix Conference 2025

## Matrix is built for decentralised, interoperable communications

As Matrix is an open standard , it is not a product or platform owned by any single vendor. Anyone can implement it. Each institution retains full control of its own infrastructure and data. This is critical as it enables each Länder and government agency to run its own server and integrate the Matrix standard in their own IT systems (“Fachverfahren"), remain digitally sovereign and be able to federate with any other Matrix-based deployment. Servers from different organisations can federate, meaning they exchange messages securely without routing data through a central third party.

In short, Matrix offers the opposite of a vendor-locked solution that would require all the Länder and government agencies to trust a shared commercial platform - a model that’s unworkable at scale and leaves too much of the public sector reliant on a single commercial vendor.

By adopting Matrix as the digital commons for ZaPuK , Germany’s federal government and 16 states will operate from an open standard that supports a competitive ecosystem of providers to drive innovation and efficiency based on interoperable open source software. Indeed, Germany’s public sector organisations can even build their own solutions purely from FOSS components.

## Next steps

The IT Planning Council has requested the Federal Ministry for Digital Transformation and Government Modernisation and FITKO to concentrate their shared architecture management resources on shaping the D-Stack and supporting its deployment. Key architectural deliverables are due to be presented in November. These include a capability map for the platform core and specifications for the D-architecture's functional components, with particular attention to D-Stack products and projects, notably ZaPuK (Zielarchitektur Postfach- und Kommunikationslösungen) and ZBDS (Zentraler Bezahldienst Deutschland-Stack).

Running in parallel with the ZaPuK architecture work, FITKO and the Federal Ministry for Digital Transformation and Government Modernisation (BMDS) are jointly delivering a live pilot project known as Neo. Its official title is " Piloting a Matrix-based Communication Infrastructure for G2C Communication ". The project provides necessary technology to extend BundID to become the first concrete implementation of the ZaPuK target architecture.

FITKO ran a formal procurement process, concluding in June 2025, awarding a contract to a consortium of adesso , Nordeck and Element. The project encompasses three Matrix-enabled key deliverables: an extension of the BundID web interface; a smartphone app for private individuals to communicate with public authorities on matters related to OZG online services; and tooling to allow authorities to connect their specialist systems (Fachanwendung) to the new infrastructure. All outputs are being developed as open, reusable solutions available to the wider public sector as a foundation for evolving existing infrastructure.

The Neo app is a tangible, working proof-of-concept for ZaPuK, demonstrating that the Matrix protocol can underpin a unified, end-to-end encrypted, real time communication layer between the German state and its citizens.

## Matrix and MLS

ZaPuK's specification of MLS (Messaging Layer Security) as its end-to-end encryption layer also represents a significant moment for the Matrix protocol itself. MLS is an encryption standard defined by the IETF - the standards body responsible for much of the foundational infrastructure of the internet. It provides a standard way for users of a messaging service to communicate securely, without servers being able to eavesdrop on their conversations.

The Matrix.org Foundation was part of the initial working group who pledged support for MLS at IETF 101 back in 2018, and has been investigating ways of integrating Matrix and MLS since 2020. The challenge is that MLS assumes a centralised model where participation is mediated by a single MLS delivery server, whereas Matrix assumes a decentralised model where Matrix servers manage participation without a single point of control. Recently, significant progress has been made in decentralising MLS and using it to manage participation, while using Matrix for decentralised access control and conversation replication. ZaPuK’s adoption of MLS should further drive this work forwards.

For a deeper dive on MLS integrating into the decentralised world of Matrix, see Matthew Hodgson’s recent Matrix Live recording .

## A successful precedent: TI-Messenger

Germany's healthcare regulator gematik, majority-owned by the Federal Ministry of Health, mandated Matrix as the foundation for TI-Messenger (TI-M), the official secure communications standard for the German healthcare system.

TI-Messenger will connect more than 150,000 healthcare organisations and 74 million patients. In July 2025, it became available to citizens covered by statutory health insurance, integrated directly into their electronic patient record apps. Over time it will replace phone calls, faxes, and disconnected portals that currently mediate so much of Germany's clinical communication.

Like ZaPuK, TI-Messenger is built on the principle that sovereignty and interoperability are not in tension. Each healthcare provider can operate its own Matrix server. Gematik certifies compliance with the standard but does not control the infrastructure. The result is a private, federated network: interoperable by design, sovereign by architecture.

With FITKO’s ZaPuK confirmed as a part of D-Stack, there are now two standards setting bodies - one responsible for healthcare infrastructure, one for cross-government digital services - that have evaluated interoperable communications use across a broad ecosystem and both have reached the same conclusion. That is: open standards, federated architecture, and data sovereignty are not aspirational principles; they are practical requirements, and Matrix is the best way to meet them.
