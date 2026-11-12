<!--
SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)

SPDX-License-Identifier: CC-BY-SA-4.0
-->

<!-- cspell:words DSGVO TDDDG ERVV Bekanntgabefiktion -->

# Delivery, Retrieval and Read Receipts

This md contains concepts for #303: which law governs the notification of a message in Neo, which event starts the deadline and what each type proves.

Following the Rechtsgutachten and ADR-0018 (Bereitstellungsbestätigung), ADR-1118 (Abruf-/Lesebestätigung), ADR-1119 (Zustelloptionen), and ADR-1120 (Anhänge)

## Legal Grounds

The legal basis is organized around **§ 9 OZG as the default**, as e.g. § 173 Abs. 4 ZPO addresses court-service law specifically (Fachrecht).

However, Fachrecht overrides § 9 OZG inside the same mailbox for certain types of communication, specifically § 122a Abs. 5 AO and § 37 Abs. 2a SGB X. Corresponding domains are listed under "laws".

## Translation/definition of terms used

| Term | English | Meaning |
| ---- | ------- | ------- |
| Verwaltungsakt (VA) | administrative decision | only those require real-world fictions |
| Bekanntgabe | notification | makes a VA effective |
| Zustellung | formal service/delivery | prescribed form, mandatory proof |
| Zugang | receipt | arrival in readable form |
| Machtbereich | sphere of control | where arrival constitutes Zugang |
| Bekanntgabefiktion | constructive notice | statutory notification date |
| Fachrecht | sector-specific law | overrides the default |
| Heilung | curing of defective service | actual receipt repairs the defect |
| Bereitstellung | provisioning | message persisted and retrievable |
| Abruf | retrieval | called up and opened to take notice |
| Empfangsbekenntnis | receipt acknowledgement | active, dated declaration |

---

## Default case: § 9 Abs. 1 OZG

Bereitstellung (provisioning) + 4 days (S. 4); authority proves provisioning and its time (S. 5); notification informational only (S. 6). Under § 9 Abs. 1 S. 7 OZG, where an authority provisions once more, an earlier retrieval keeps the day of first retrieval as the date of "Zugang" — meaning it does not shorten the initial four days, and only where the authority can establish the retrieval in court.

Same applies to:

- Finanzverwaltung/ELSTER (§ 122a Abs. 4 AO)
- Elterngeld (§ 9 OZG, § 37 Abs. 2b SGB X)
- MJP, specifically court → citizen (§ 173 Abs. 4 S. 4 ZPO)

→ ADR-0018

## Exceptions

**§ 37 Abs. 2a S. 4–7 SGB X: Sozialleistungsträger** (Rentenversicherung, Krankenkassen, etc.). In this case, dispatch of the notification (instead of provisioning) must be proven → the authority must prove the notification arrived (S. 5), otherwise the retrieval day counts (S. 6+7). Since the notification goes out on the provisioning day, consequently, the calculated date is the same.

**Acknowledgement instead of fiction** (§ 173 Abs. 3 ZPO; court → authorities (G2G), court experts, etc., and § 5 Abs. 7 S. 1 VwZG). The latter is covered by the first. Important: Not a G2C case !-- see Lesebestätigung.

**No fiction needed at all:** documents that are not Verwaltungsakte (Anhörungen, Zwischennachrichten, Mitwirkungsaufforderungen)

## Extensions to be discussed when § 9 OZG changes

§ 41 Abs. 2a S. 3–5 VwVfG: here, retrieval is binding and must be logged, 10-day termination.

## Confirmation types

**Bereitstellungsbestätigung** starts the fiction in the default and AO case; it proves Bereitstellung only (yet not Zugang). the confirmation evidences arrival of the ciphertext, not its ability to decrypt. Its evidence rests on being issued by the recipient's backend rather than the sender itself.

**Abrufbestätigung (retrieval receipt)** never starts the fiction but:

- keeps the day of first retrieval where a Bescheid is notified again (§ 9 Abs. 1 S. 7 OZG), where provable in court
- is constitutive under § 41 Abs. 2a VwVfG
- is fallback under SGB X (§ 37 Abs. 2a S. 6–7)
- is the only receipt evidence for documents that are not Verwaltungsakte
- supports Heilung under § 189 ZPO / § 8 VwZG (D.VI.4.5: "Hilfsmittel zur Heilung, nicht Ersatz des Empfangsbekenntnisses")

It is not an Empfangsbekenntnis (D.VI.4.2: "untauglich, weil ihr das Willenselement fehlt"). Suppressible by a non-conformant client, but following D.II.2.2.2.b: "Das Ausbleiben … wäre für die Behörde jedoch nachvollziehbar". Ergo, the sender chooses whether to request it.

**Lesebestätigung (read receipt)** qualifies as Empfangsbekenntnis if actively given with a freely entered date ; the encrypted event alone is insufficient for court use, which requires the structured XJustiz dataset and its return; no QES and no elevated trust level while the mailbox owner sends; not required for citizens (D.VI.4.7).
Only for recipients under § 173 Abs. 2 ZPO and § 5 VwZG (**not a G2C case**).

**Eingangsbestätigung** (§ 130a Abs. 5 S. 2 ZPO) is out of scope as something Neo produces. it is issued by the court and received via the protocol converter.

**Löschquittung (deletion receipt)** referring to a deleted not retrieved Bescheid, which triggers renewed Bekanntgabe (§ 9 Abs. 1 S. 7 OZG) or termination (§ 41 Abs. 2a S. 4–5 VwVfG).

## Requirements regardless of their legal base

- **Zugang:** "in für den Empfänger bearbeitbarer Weise" (vgl. § 130 Abs. 1 BGB; § 87a Abs. 1 S. 3 AO)
- **Readability:** "…ist eine funktionierende Schlüsselwiederherstellung die Bedingung dafür, dass die Fiktion überhaupt eingreifen kann" (D.VII.1); ADR-1134. A failed decryption should be reportable so the sender can provision again before the fiction takes effect (§ 3a Abs. 4 S. 2 VwVfG, § 87a Abs. 2 S. 2 AO).
- **Attachments:** the attachment must reside on the recipient backend. Route is irrelevant, only a file left in the sender's media repository and merely linked is not provisioned (§ 9 Abs. 1 S. 1 OZG).
- **Seal:** an advanced seal is the right form for a machine-generated server receipt, but a key merely published in the SSP is probably not sufficient following Rechtsgutachten — "erforderlich ist eine Zertifikatsbindung an den Betreiber als juristische Person" (Art. 36 eIDAS; D.VI.2.4). Neither a qualified seal nor a qualified timestamp is mandatory (D.VI.2); a qualified timestamp is recommended.
- **Retention:** no legislative duty to keep messages in the mailbox; deletion follows purpose expiry per data type (Art. 5 Abs. 1 lit. e DSGVO), traffic data under § 9 TDDDG (D.V.5). Confirmations are not deleted before the authority has exported the originals into its record.

→ Since the laws above differ by sending authority, each message might need to carry which law applies to it so the deadline logic can be applied correctly. The Rechtsgutachten requires the relevant events to be documented.

---

## Laws

### OZG

- § 2 OZG: Nutzerkonto (Abs. 5), Postfach (Abs. 7) → <https://www.gesetze-im-internet.de/ozg/__2.html>
- § 9 OZG: default case → <https://www.gesetze-im-internet.de/ozg/__9.html>

### AO

- § 87a AO: processable form (Abs. 1 S. 3), re-transmission duty (Abs. 2 S. 2) → <https://www.gesetze-im-internet.de/ao_1977/__87a.html>
- § 122 AO: general Bekanntgabe → <https://www.gesetze-im-internet.de/ao_1977/__122.html>
- § 122a AO: ELSTER (applies in the OZG mailbox) → <https://www.gesetze-im-internet.de/ao_1977/__122a.html>

### SGB X

- § 37 SGB X: Abs. 2a Sozialleistungsträger, Abs. 2b Elterngeld → <https://www.gesetze-im-internet.de/sgb_10/__37.html>

### VwVfG

- § 3a VwVfG: re-transmission where the document cannot be processed (Abs. 4 S. 2) → <https://www.gesetze-im-internet.de/vwvfg/__3a.html>
- § 41 VwVfG: Abs. 2 transmission model, Abs. 2a retrieval model → <https://www.gesetze-im-internet.de/vwvfg/__41.html>

### VwZG

- § 5 VwZG: formal service, Empfangsbekenntnis (Abs. 7) → <https://www.gesetze-im-internet.de/vwzg_2005/__5.html>
- § 8 VwZG: Heilung → <https://www.gesetze-im-internet.de/vwzg_2005/__8.html>

### ZPO

- § 130a ZPO: Eingangsbestätigung (Abs. 5 S. 2), secure transmission path (Abs. 4 S. 1 Nr. 4) → <https://www.gesetze-im-internet.de/zpo/__130a.html>
- § 173 ZPO: Abs. 3 Empfangsbekenntnis, Abs. 4 fiction for citizens → <https://www.gesetze-im-internet.de/zpo/__173.html>
- § 189 ZPO: Heilung → <https://www.gesetze-im-internet.de/zpo/__189.html>
- § 371a ZPO: evidential weight of electronic documents → <https://www.gesetze-im-internet.de/zpo/__371a.html>

### BGB

- § 130 BGB: Zugang, applied analogously → <https://www.gesetze-im-internet.de/bgb/__130.html>

### ERVV

- § 13 ERVV: secure transmission path via a Nutzerkonto → <https://www.gesetze-im-internet.de/ervv/__13.html>
