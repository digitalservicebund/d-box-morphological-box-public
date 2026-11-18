# beBPo – Voraussetzungen – EGVP (egvp.justiz.de)

Quelle: https://egvp.justiz.de/behoerdenpostfach/Voraussetzungen/index.php
Abgerufen: 2026-11-18 über Internet Archive (Originalserver nicht erreichbar): https://web.archive.org/web/20251111114357id_/https://egvp.justiz.de/behoerdenpostfach/Voraussetzungen/index.php

---



Technische und organisatorische Voraussetzungen

I. Welche Sende- und Empfangssoftwarelösungen können genutzt werden?

   Als Sende- und Empfangssoftware für das beBPo stehen eine Reihe von
   Softwarelösungen zur Verfügung. Eine Übersicht der Produkte finden Sie
   auf der Seite zu den Informationen zu Drittanwendungen.

   Darüber hinaus stellt die Justiz den Behörden kostenlos die Software
   EGVP-Enterprise zur Verfügung, die zur Integration in die Fachverfahren
   genutzt werden kann. Die EGVP-Enterprise wird bspw. bereits vom
   Bundesamt für Migration und Flüchtlinge, der Deutschen
   Rentenversicherung und der Finanzverwaltung eingesetzt. Sie ist nicht
   für die Nutzung am lokalen Arbeitsplatz geeignet. Für die
   Bereitstellung der EGVP-Enterprise steht unter
   www.egvp.de/Drittprodukte/EGVP_Registrierungsverfahren_Teilnahme_Drittp
   rodukte_Antragsformular_Enterprise.doc  Word-Dokument, öffnet neues
   Browserfenster  ein Formular zur Verfügung.

II.            Wer stellt einen Intermediär zur Verfügung?

   Für die EGVP-Infrastruktur werden für die „Zwischenspeicherung“ der
   Nachrichten (ähnlich einem E-Mail-Server) sogenannte Intermediäre (auch
   virtuelle Poststellen genannt), benötigt. Diese werden in allen
   Bundesländern und auf Bundesebene in öffentlich-rechtlichen
   Rechenzentren betrieben.

   Bitten wenden Sie sich an den Intermediärsbetreiber Ihres Bundeslandes.

   Es besteht auch die Möglichkeit, einen eigenen Intermediär zu
   betreiben. Die Software steht dem Bund und den Ländern als Anwendung
   des IT-Planungsrates im Rahmen der einschlägigen Pflegeverträge zur
   Verfügung. Für Detail-Informationen steht die Governikus KG unter
   kontakt@governikus.de zur Verfügung.

III.          Wie kann ein eigener Verzeichnisdienst für die beBPos
eingerichtet werden?

   Die beBPos müssen in sicheren Verzeichnisdiensten, die dem
   SAFE-Standard entsprechen, eingetragen werden. Die Verzeichnisdienste
   dienen zum einen zur eindeutigen Identifizierung von beBPos, zum
   anderen werden sie benötigt, um beBPos finden und adressieren zu
   können.

   In den Ländern kann für die beBPos ein eigener SAFE-konformer
   Verzeichnisdienst aufgebaut werden. Die Software SAFE der Justiz wird
   dafür kostenlos bereitgestellt.

   Die Einbindung solcher weiteren Verzeichnisdienste in die
   EGVP-Infrastruktur wird von der BLK-AG IT-Standards in der Justiz
   koordiniert.

   Für Detailinformationen steht das Projektbüro EGVP unter
   it-standards@justiz.de als Ansprechpartner zur Verfügung.

IV. Kann auch ein bestehender sicherer Verzeichnisdienst genutzt werden?

   Für die EGVP-Infrastruktur sind bereits folgende sicheren
   SAFE-konformen Verzeichnisdienste eingerichtet:
     * Verzeichnisdienst der Justiz (Softwaresystem SAFE) für die
       EGVP-Postfächer der Justiz und der Verwaltungsbehörden,
     * Verzeichnisdienst der Bundesrechtsanwaltskammer (Softwaresystem
       Governikus autent)  für das besondere elektronische Anwaltspostfach
       (beA) und
     * Verzeichnisdienst der Bundesnotarkammer (Softwaresystem procilon
       proNEXT Security Manager) für das besondere elektronische
       Notarpostfach (beN)

    Für die beBPos kann der SAFE-Verzeichnisdienst der Justiz genutzt
   werden.

V. Wer pflegt die beBPos in den Verzeichnisdienst?

   Gemäß den Regelungen des ERVV müssen von den obersten Behörden des
   Bundes oder der Länder öffentlich-rechtliche Stellen bestimmt werden,
   die die Identität der Behörden oder juristischen Personen des
   öffentlichen Rechts prüfen und dies im Verzeichnisdienst bestätigen (im
   Folgenden „beBPo-Prüfstellen“ genannt).

   Sofern der SAFE-Verzeichnisdienst der Justiz genutzt wird, werden die
   beBPos dort von den beBPo-Prüfstellen – oder in deren Auftrag – selbst
   verwaltet (Freischaltung, Löschung, Pflege). Hierfür wird ein
   Administrationszugriff auf den SAFE-Verzeichnisdienst eingerichtet, der
   auf die jeweiligen beBPos beschränkt ist.  Es ist somit möglich,
   sozusagen einen „eigenen Verzeichnisdienst“ innerhalb des
   SAFE-Verzeichnisdienstes der Justiz zu betreiben.

VI. Wie ist die Herkunft einer Nachricht aus einem beBPo erkennbar?

   beBPos werden im sicheren Verzeichnisdienst als solche gekennzeichnet.
   (Sie erhalten die sogenannte EGVP-Rolle „beBPo“. Das EGVP-Rollenkonzept
   ist in der Anlage erläutert.)

   Zudem wird an jede EGVP-Nachricht, die aus einem beBPo versandt wird,
   automatisiert eine bestimmte Signatur an den äußeren Umschlag
   (Transportsignatur) angebracht. Diese bestätigt die Herkunft der
   Nachricht aus dem beBPo (Herkunftsnachweis).

   Diese Transportsignatur, die als Herkunftsnachweis dient, wird
   maschinell geprüft. Das fortgeschrittene Signaturzertifikat muss von
   einem bestimmten Ausstellerzertifikat abstammen, das für den besonderen
   Zweck, die Herkunft einer Nachricht aus einem beBPo zu bestätigen,
   ausgestellt wurde. Das maschinell erstellte Prüfprotokoll und die
   übermittelten Dokumente werden zur Gerichtsakte genommen.

VII.         Wo können fortgeschrittene Signaturzertifikate, die für die
Anbringung des Herkunftsnachweises genutzt werden können, beschafft werden?

   Für den Download von beBPo-VHN-Zertifikaten wird im Auftrag der
   Justiz unter https://zertifikate.safe-justiz.de/UserCertificateManageme
   ntUI/#!/creation  externer Link, öffnet neues Browserfenster  eine
   Web-Anwendung bereitgestellt.

   An dieser Webanwendung können sich Behörden/Körperschaften öffentlichen
   Rechts selbst  anmelden und ein Zertifikat herunterladen, sobald sie
   • ein beBPo eingerichtet haben,
   • dieses von der beBPo-Prüfstelle authentifiziert wurde, und
   • die Vergabe der Rolle egvp_beBPo im SAFE-Verzeichnisdienst erfolgt
   ist.

   Einzelheiten zur Funktionsweise der Web-Anwendung sind im dort
   veröffentlichten Leitfaden dargestellt.

   Die Rolle egvp_beBPo erhält eine Behörde/Körperschaft öffentlichen
   Rechts als Bestätigung der Authentifizierung ihres besonderen
   Behördenpostfaches nach den Vorgaben der ERVV. Diese Authentifizierung
   wirkt somit für die Beantragung eines beBPo-VHN-Zertifikats fort.

   Druckvorschau in neuem Fenster öffnen  Seite drucken  |  zum
   Seitenanfang gehen  Seitenanfang

Zusätzliche Informationen

