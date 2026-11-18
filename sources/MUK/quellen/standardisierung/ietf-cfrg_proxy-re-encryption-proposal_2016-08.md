# [Cfrg] Proposal to use Proxy Re-Encryption in a messaging protocol

Quelle: https://mailarchive.ietf.org/arch/msg/cfrg/xSuAKoMXygK9JYbQnHZ0HtYSe-k/
Abgerufen: 2026-11-18

```text
   Archived-At:
   <https://mailarchive.ietf.org/arch/msg/cfrg/xSuAKoMXygK9JYbQnHZ0HtYSe-k
   >
   Subject: [Cfrg] Proposal to use Proxy Re-Encryption in a messaging
   protocol
   X-BeenThere: cfrg@irtf.org
   X-Mailman-Version: 2.1.17
   Precedence: list
   List-Id: Crypto Forum Research Group <cfrg.irtf.org>
   List-Unsubscribe: <https://www.irtf.org/mailman/options/cfrg>,
   <mailto:cfrg-request@irtf.org?subject=unsubscribe>
   List-Archive: <https://mailarchive.ietf.org/arch/browse/cfrg/>
   List-Post: <mailto:cfrg@irtf.org>
   List-Help: <mailto:cfrg-request@irtf.org?subject=help>
   List-Subscribe: <https://www.irtf.org/mailman/listinfo/cfrg>,
   <mailto:cfrg-request@irtf.org?subject=subscribe>
   X-List-Received-Date: Mon, 22 Aug 2016 19:04:12 -0000
NB: Please direct followups to endymail@ietf.org alone


The draft is at:

https://www.ietf.org/id/draft-hallambaker-mesh-recrypt-00.txt


At the last IETF, I made a presentation on the use of Proxy Re-Encryption
'recryption' at the CFRG session=. I think that this is a very powerful
technique that solves some real problems we are facing today that were
probably not as apparent when it was first proposed.

In particular, recryption allows end-to-end security to be preserved in
situations where it would normally be lost. For example in a mailing list
application or in a situation where Alice needs to read her email on
multiple devices, some of which might be mobile devices that could get
lost. Recryption also provides the ideal basis for Confidential Document
Control which is an access control system that uses data level encryption,

One slight holdup here is that there is a patent encumbrance that purports
to claim the use of recyption for DRM applications but this will expire
shortly, certainly before any project could get off the ground.


I have written an Internet draft showing how Recryption might be
implemented as a 'clean slate' protocol. Since we don't have anything like
a CDC application yet (Plasma maybe), this is going to be a requirement for
some situations. I am thinking we should probably try to build something
and work out how to get that running before working out how best to fit
these capabilities to S/MIME, OpenPGP, Jabber, etc.

Contrary to my usual practice, there is no code so far, well no
implementation code.I will be filling that in once I finish a few things
ahead of this in the queue, specifically using the Mesh to manage SSH keys.


The one technical holdup I see here is that if we are going to get people
to use it, usability can't be 'OK' or 'not bad'. The only way to get a new
crypto system off the ground is to design something that delivers usability
that is iPhone level perfect. I think that the Mesh makes that possible of
course but I will probably have to prove that with some demos. Which is why
I want to get the Mesh to manage SSH keys.

     * [Cfrg] Proposal to use Proxy Re-Encryption in a m…  Phillip
       Hallam-Baker

     * Hide Navigation Bar

   Date
   Thread
   (BUTTON)
     *
     * Date
     *
     *
     * Thread
     *

   v2.46.0 | Report a Bug | By Email | System Status
```
