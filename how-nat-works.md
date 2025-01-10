How Network Address Translator (NAT) works | Tailscale:root{--font-inter:'\_\_Inter\_81dec9', '\_\_Inter\_Fallback\_81dec9';--font-mdio:'\_\_MDIOFont\_8d6c39', '\_\_MDIOFont\_Fallback\_8d6c39'}

[Say goodbye to your legacy VPNMake the switch to Tailscale](https://tailscale.com/switch)

[](/)

Product

Solutions

[Enterprise

](/enterprise)[Customers

](/customers)[Docs

](/kb/1017/install)[Blog

](/blog)[Pricing

](/pricing)

[Download](/download)[Log in](https://login.tailscale.com/welcome)[

Get started

](https://login.tailscale.com/start)

Product

Meet Tailscale

* [

  How it works

  ](/blog/how-tailscale-works)
* [

  Why Tailscale

  ](/why-tailscale)
* [

  WireGuard® for Enterprises

  ](/wireguard-vpn)
* [

  Bring Tailscale to Work

  ](/bring-tailscale-to-work)

Explore

* [

  Integrations

  ](/integrations)
* [

  Features

  ](/features)
* [

  Compare Tailscale

  ](/compare)
* [

  Partnerships

  ](/partnerships)

Solutions

By use-case

* [

  Remote Access

  ](/use-cases/remote-access)
* [

  Site-to-site Networking

  ](/use-cases/site-to-site-networking)
* [

  Multi-Cloud Networking

  ](/use-cases/multi-cloud-networking)
* [

  Kubernetes Networking

  ](/use-cases/kubernetes)
* [

  Edge & IoT Deployments

  ](/use-cases/iot)
* [

  Zero Trust Networking

  ](/use-cases/zero-trust-networking)
* [

  AI Workloads

  ](/use-cases/ai)
* [

  Secure SaaS

  ](/use-cases/secure-saas)
* [

  Business VPN

  ](/use-cases/business-vpn)
* [

  Homelab

  ](/use-cases/homelab)

By role

* [

  DevOps

  ](/solutions/devops)
* [

  IT

  ](/solutions/it)
* [

  Security

  ](/solutions/security)

[Enterprise](/enterprise)

[Customers](/customers)

Nav heading here

* [

  <img alt="Alt text " src="https://cdn.sanity.io/images/w77i7m8x/production/a06dc612b1e3e4f4df53a72030002600639a8738-300x120.png?w=640&q=75&fit=clip&auto=format" height="120" width="300" />

  Title here

  How Cribl Enables Secure Work From Anywhere with Tailscale

  ](https://tailscale.com/customers)

[Docs](/kb/1017/install)

[Blog](/blog)

[Pricing](/pricing)

[Download](/download)

[

Get started - it's free!

](https://login.tailscale.com/start)[

Login

](https://login.tailscale.com/welcome)

WireGuard is a registered trademark of Jason A. Donenfeld.

[Terms of Service](/terms)[Privacy Policy](/privacy-policy)

 © 2025 Tailscale Inc. All rights reserved. Tailscale is a registered trademark of Tailscale Inc.

{"@context":"https://schema.org","@type":"BlogPosting","image":"https://cdn.sanity.io/images/w77i7m8x/production/f94042c9459e89db55090dc0203b042bdc2c5937-600x315.svg","url":"https://tailscale.com/blog/how-nat-traversal-works","headline":"How NAT traversal works","datePublished":"2020-08-21T00:00:00Z","description":"Here we cover how we can get through NATs (Network Address Translators) and connect your devices directly to each other, no matter what’s standing between them. ","abstract":"Here we cover how we can get through NATs (Network Address Translators) and connect your devices directly to each other, no matter what’s standing between them. ","author":[{"@type":"Person","name":"David Anderson"}],"articleBody":"\\n \\n .Markdown p \> code {\\n border: none;\\n -webkit-font-smoothing: subpixel-antialiased;\\n -moz-osx-font-smoothing: auto;\\n }\\n\\n

We covered a lot of ground in our post about [*How Tailscale\\nWorks*](\"/blog/how-tailscale-works\"). However, we glossed over how we can get through NATs\\n(Network Address Translators) and connect your devices directly to\\neach other, no matter what’s standing between them. Let’s talk about\\nthat now!

\\n \\n \\n\\n\\n ![\"NAT](\"https://cdn.sanity.io/images/w77i7m8x/production/50eafc1638d93b9637dcee0d55967a8fa09e05c7-1700x800.png\")\\n \\n\\n\\n

Let’s start with a simple problem: establishing a peer-to-peer\\nconnection between two machines. In Tailscale’s case, we want to set\\nup a WireGuard® tunnel, but that doesn’t really matter. The\\ntechniques we use are widely applicable and the work of many people\\nover decades. For example, [WebRTC](\"https://webrtc.org/\") uses this bag of tricks to\\nsend peer-to-peer audio, video and data between web browsers. VoIP\\nphones and some video games use similar techniques, though not always\\nsuccessfully.

\\n

We’ll be discussing these techniques generically, using Tailscale and\\nothers for examples where appropriate. Let’s say you’re making your\\nown protocol and that you want NAT traversal. You need two things.

\\n

First, the protocol should be based on UDP. You *can* do NAT traversal\\nwith TCP, but it adds another layer of complexity to an already quite\\ncomplex problem, and may even require kernel customizations depending\\non how deep you want to go. We’re going to focus on UDP for the rest\\nof this article.

\\n

If you’re reaching for TCP because you want a stream-oriented\\nconnection when the NAT traversal is done, consider using QUIC\\ninstead. It builds on top of UDP, so we can focus on UDP for NAT\\ntraversal and still have a nice stream protocol at the end.

\\n

Second, you need direct control over the network socket that’s sending\\nand receiving network packets. As a rule, you can’t take an existing\\nnetwork library and make it traverse NATs, because you have to send\\nand receive extra packets that aren’t part of the “main” protocol\\nyou’re trying to speak. Some protocols tightly integrate the NAT\\ntraversal with the rest (e.g. WebRTC). But if you’re building your\\nown, it’s helpful to think of NAT traversal as a separate entity that\\nshares a socket with your main protocol. Both run in parallel, one\\nenabling the other.

\\n\\n \\n \\n\\n\\n ![\"NAT](\"https://cdn.sanity.io/images/w77i7m8x/production/5bef4a143b33f1a5b2ce49e919dccb4e59b7569a-1600x760.png\")\\n \\n\\n\\n

Direct socket access may be tough depending on your situation. One\\nworkaround is to run a local proxy. Your protocol speaks to this\\nproxy, and the proxy does both NAT traversal and relaying of your\\npackets to the peer. This layer of indirection lets you benefit from\\nNAT traversal without altering your original program.

\\n

With prerequisites out of the way, let’s go through NAT traversal from\\nfirst principles. Our goal is to get UDP packets flowing\\nbidirectionally between two devices, so that our other protocol\\n(WireGuard, QUIC, WebRTC, …) can do something cool. There are two\\nobstacles to having this Just Work: stateful firewalls and NAT\\ndevices.

\\n

### Figuring out firewalls ###

\\n

Stateful firewalls are the simpler of our two problems. In fact, most\\nNAT devices include a stateful firewall, so we need to solve this\\nsubset before we can tackle NATs.

\\n

There are many incarnations to consider. Some you might recognize are\\nthe Windows Defender firewall, Ubuntu’s ufw (using iptables/nftables),\\nBSD’s pf (also used by macOS) and AWS’s Security Groups. They’re all\\nvery configurable, but the most common configuration allows all\\n“outbound” connections and blocks all “inbound” connections. There\\nmight be a few handpicked exceptions, such as allowing inbound SSH.

\\n

But connections and “direction” are a figment of the protocol\\ndesigner’s imagination. On the wire, every connection ends up being\\nbidirectional; it’s all individual packets flying back and forth. How\\ndoes the firewall know what’s inbound and what’s outbound?

\\n

That’s where the stateful part comes in. Stateful firewalls remember\\nwhat packets they’ve seen in the past and can use that knowledge when\\ndeciding what to do with new packets that show up.

\\n\\n \\n \\n\\n\\n ![\"NAT](\"https://cdn.sanity.io/images/w77i7m8x/production/2b44043b374217d6fa2d3a138b77c171df05bdf2-1600x740.png\")\\n \\n\\n\\n

For UDP, the rule is very simple: the firewall allows an inbound UDP\\npacket if it previously saw a matching outbound packet. For example,\\nif our laptop firewall sees a UDP packet leaving the laptop from\\n`2.2.2.2:1234` to `7.7.7.7:5678`, it’ll make a note that incoming\\npackets from `7.7.7.7:5678` to `2.2.2.2:1234` are also fine. The\\ntrusted side of the world clearly intended to communicate with\\n`7.7.7.7:5678`, so we should let them talk back.

\\n

(As an aside, some *very* relaxed firewalls might allow traffic from\\nanywhere back to `2.2.2.2:1234` once `2.2.2.2:1234` has communicated\\nwith anyone. Such firewalls make our traversal job easier, but are\\nincreasingly rare.)

\\n

#### Firewall face-off ####

\\n

This rule for UDP traffic is only a minor problem for us, as long as\\nall the firewalls on the path are “facing” the same way. That’s\\nusually the case when you’re communicating with a server on the\\ninternet. Our only constraint is that the machine that’s *behind* the\\nfirewall must be the one initiating all connections. Nothing can\\ntalk to it, unless it talks first.

\\n\\n \\n \\n\\n\\n ![\"NAT](\"https://cdn.sanity.io/images/w77i7m8x/production/e4277f239f9dbd1344451fc7d5e7da067447fd4d-2100x788.png\")\\n \\n\\n\\n

This is fine, but not very interesting: we’ve reinvented client/server\\ncommunication, where the server makes itself easily reachable to\\nclients. In the VPN world, this leads to a hub-and-spoke topology: the\\nhub has no firewalls blocking access to it and the firewalled spokes\\nconnect to the hub.

\\n\\n \\n \\n\\n\\n ![\"VPN](\"https://cdn.sanity.io/images/w77i7m8x/production/2bbe8d0fa6f3a66b71f0aec01bf831a3cc1a8a65-2210x1082.png\")\\n \\n\\n\\n

The problems start when two of our “clients” want to talk\\ndirectly. Now the firewalls are facing each other. According to the\\nrule we established above, this means both sides must go first, but\\nalso that neither can go first, because the other side has to go\\nfirst!

\\n\\n \\n \\n\\n\\n ![\"Diagram](\"https://cdn.sanity.io/images/w77i7m8x/production/928409c960e0b0bcd53560edf80a934b24eaec11-1740x620.png\")\\n \\n\\n\\n

How do we get around this? One way would be to require users to\\nreconfigure one or both of the firewalls to “open a port” and allow\\nthe other machine’s traffic. This is not very user friendly. It also\\ndoesn’t scale to mesh networks like Tailscale, in which we expect the\\npeers to be moving around the internet with some regularity. And, of\\ncourse, in many cases you don’t have control over the firewalls: you\\ncan’t reconfigure the router in your favorite coffee shop, or at the\\nairport. (At least, hopefully not!)

\\n

We need another option. One that doesn’t involve reconfiguring\\nfirewalls.

\\n

#### Finessing finicky firewalls ####

\\n

The trick is to carefully read the rule we established for our\\nstateful firewalls. For UDP, the rule is: **packets must flow out\\nbefore packets can flow back in.**

\\n

However, nothing says the packets must be *related* to each other\\nbeyond the IPs and ports lining up correctly. As long as *some* packet\\nflowed outwards with the right source and destination, any packet that\\n*looks like* a response will be allowed back in, even if the other\\nside never received your packet!

\\n

So, to traverse these multiple stateful firewalls, we need to share\\nsome information to get underway: the peers have to know in advance\\nthe `ip:port` their counterpart is using. One approach is to\\nstatically configure each peer by hand, but this approach doesn’t\\nscale very far. To move beyond that, we built a [coordination\\nserver](\"/blog/how-tailscale-works#the-control-plane-key-exchange-and-coordination\") to keep the `ip:port` information synchronized in a\\nflexible, secure manner.

\\n

Then, the peers start sending UDP packets to each other. They must\\nexpect some of these packets to get lost, so they can’t carry any\\nprecious information unless you’re prepared to retransmit them. This\\nis generally true of UDP, but especially true here. We’re *going* to\\nlose some packets in this process.

\\n

Our laptop and workstation are now listening on fixed ports, so that\\nthey both know exactly what `ip:port` to talk to. Let’s take a look at\\nwhat happens.

\\n\\n \\n \\n\\n\\n ![\"Diagram](\"https://cdn.sanity.io/images/w77i7m8x/production/d7310815a3e9f715b549b2043f60b94b04b42b6d-1740x680.png\")\\n \\n\\n\\n

The laptop’s first packet, from `2.2.2.2:1234` to `7.7.7.7:5678`, goes\\nthrough the Windows Defender firewall and out to the internet. The\\ncorporate firewall on the other end blocks the packet, since it has no\\nrecord of `7.7.7.7:5678` ever talking to `2.2.2.2:1234`. However,\\nWindows Defender now remembers that it should expect and allow\\nresponses from `7.7.7.7:5678` to `2.2.2.2:1234`.

\\n\\n \\n \\n\\n\\n ![\"Packet](\"https://cdn.sanity.io/images/w77i7m8x/production/9cfd8653ec918a72d6909e5603d65c2ca4b6e5c9-1740x680.png\")\\n \\n\\n\\n

Next, the workstation’s first packet from `7.7.7.7:5678` to\\n`2.2.2.2:1234` goes through the corporate firewall and across the\\ninternet. When it arrives at the laptop, Windows Defender thinks “ah,\\na response to that outbound request I saw”, and lets the packet\\nthrough! Additionally, the corporate firewall now remembers that it\\nshould expect responses from `2.2.2.2:1234` to `7.7.7.7:5678`, and\\nthat those packets are also okay.

\\n

Encouraged by the receipt of a packet from the workstation, the laptop\\nsends another packet back. It goes through the Windows Defender\\nfirewall, through the corporate firewall (because it’s a “response” to\\na previously sent packet), and arrives at the workstation.

\\n\\n \\n \\n\\n\\n ![\"NAT](\"https://cdn.sanity.io/images/w77i7m8x/production/7189e9a0b2caf65998dc2e50e84e8f4a35e73bbb-1740x680.png\")\\n \\n\\n\\n

Success! We’ve established two-way communication through a pair of\\nfirewalls that, at first glance, would have prevented it.

\\n

#### Creative connectivity caveats ####

\\n

It’s not always so easy. We’re relying on some indirect influence over\\nthird-party systems, which requires careful handling. What do we need\\nto keep in mind when managing firewall-traversing connections?

\\n

Both endpoints must attempt communication at roughly the same time, so\\nthat all the intermediate firewalls open up while both peers are still\\naround. One approach is to have the peers retry continuously, but this\\nis wasteful. Wouldn’t it be better if both peers knew to start\\nestablishing a connection at the same time?

\\n

This may sound a little recursive: to communicate, first you need to\\nbe able to communicate. However, this preexisting “side channel”\\ndoesn’t need to be very fancy: it can have a few seconds of latency,\\nand only needs to deliver a few thousand bytes in total, so a tiny VM\\ncan easily be a matchmaker for thousands of machines.

\\n

In the distant past, I used XMPP chat messages as the side channel,\\nwith great results. As another example, WebRTC requires you to come up\\nwith your own “signalling channel” (a name that reveals WebRTC’s IP\\ntelephony ancestry), and plug it into the WebRTC APIs. In Tailscale,\\nour coordination server and fleet of DERP (Detour Encrypted Routing\\nProtocol) servers act as our side channel.

\\n

Stateful firewalls have limited memory, meaning that we need periodic\\ncommunication to keep connections alive. If no packets are seen for a\\nwhile (a common value for UDP is 30 seconds), the firewall forgets\\nabout the session, and we have to start over. To avoid this, we use a\\ntimer and must either send packets regularly to reset the timers, or\\nhave some out-of-band way of restarting the connection on demand.

\\n

On the plus side, one thing we *don’t* need to worry about is exactly\\nhow many firewalls exist between our two peers. As long as they are\\nstateful and allow outbound connections, the simultaneous transmission\\ntechnique will get through any number of layers. That’s really nice,\\nbecause it means we get to implement the logic once, and it’ll work\\neverywhere.

\\n

…Right?

\\n

Well, not quite. For this to work, our peers need to know in advance\\nwhat `ip:port` to use for their counterparts. This is where NATs come\\ninto play, and ruin our fun.

\\n

### The nature of NATs ###

\\n

We can think of NAT (Network Address Translator) devices as stateful\\nfirewalls with one more really annoying feature: in addition to all\\nthe stateful firewalling stuff, they also alter packets as they go\\nthrough.

\\n

A NAT device is anything that does any kind of\\nNetwork Address Translation, i.e. altering the source or destination\\nIP address or port. However, when talking about connectivity problems\\nand NAT traversal, all the problems come from Source NAT, or SNAT for\\nshort. As you might expect, there is also DNAT (Destination NAT), and\\nit’s very useful but not relevant to NAT traversal.

\\n

The most common use of SNAT is to connect many devices to the\\ninternet, using fewer IP addresses than the number of devices. In the\\ncase of consumer-grade routers, we map all devices onto a single\\npublic-facing IP address. This is desirable because it turns out that\\nthere are way more devices in the world that want internet access,\\nthan IP addresses to give them (at least in IPv4 — we’ll come to IPv6\\nin a little bit). NATs let us have many devices sharing a single IP\\naddress, so despite the global shortage of IPv4 addresses, we can\\nscale the internet further with the addresses at hand.

\\n

#### Navigating a NATty network ####

\\n

Let’s look at what happens when your laptop is connected to your home\\nWi-Fi and talks to a server on the internet.

\\n\\n \\n \\n\\n\\n ![\"Diagram](\"https://cdn.sanity.io/images/w77i7m8x/production/6eb196d17f6a5fc312a5ed657f6f5a2a47213cf3-2000x760.png\")\\n \\n\\n\\n

Your laptop sends UDP packets from `192.168.0.20:1234` to\\n`7.7.7.7:5678`. This is exactly the same as if the laptop had a public\\nIP. But that won’t work on the internet: `192.168.0.20` is a private\\nIP address, which appears on many different peoples’ private\\nnetworks. The internet won’t know how to get responses back to us.

\\n

Enter the home router. The laptop’s packets flow through the home\\nrouter on their way to the internet, and the router sees that this is\\na new session that it’s never seen before.

\\n

It knows that `192.168.0.20` won’t fly on the internet, but it can\\nwork around that: it picks some unused UDP port on its own public IP\\naddress — we’ll use `2.2.2.2:4242` — and creates a *NAT mapping* that\\nestablishes an equivalence: `192.168.0.20:1234` on the LAN side is the\\nsame as `2.2.2.2:4242` on the internet side.

\\n

From now on, whenever it sees packets that match that mapping, it will rewrite\\nthe IPs and ports in the packet appropriately.

\\n\\n \\n \\n\\n\\n ![\"Diagram](\"https://cdn.sanity.io/images/w77i7m8x/production/953404277d143f8e1ece8df72697208593faccb0-2080x640.png\")\\n \\n\\n\\n

Resuming our packet’s journey: the home router applies the NAT mapping\\nit just created, and sends the packet onwards to the internet. Only\\nnow, the packet is from `2.2.2.2:4242`, not `192.168.0.20:1234`. It\\ngoes on to the server, which is none the wiser. It’s communicating\\nwith `2.2.2.2:4242`, like in our previous examples sans NAT.

\\n

Responses from the server flow back the other way as you’d expect,\\nwith the home router rewriting `2.2.2.2:4242` back to\\n`192.168.0.20:1234`. The laptop is *also* none the wiser, from its\\nperspective the internet magically figured out what to do with its\\nprivate IP address.

\\n

Our example here was with a home router, but the same principle\\napplies on corporate networks. The usual difference there is that the\\nNAT layer consists of multiple machines (for high availability or\\ncapacity reasons), and they can have more than one public IP address,\\nso that they have more public `ip:port` combinations to choose from\\nand can sustain more active clients at once.

\\n\\n \\n \\n\\n\\n ![\"Multiple](\"https://cdn.sanity.io/images/w77i7m8x/production/cdd34c97cc748ad3a478656650dc5c3f6091dc12-2300x1076.png\")\\n \\n \\n

Multiple NATs on a single layer allow for higher availability or capacity, but function the same as a single NAT.

\\n \\n\\n\\n

#### A study in STUN ####

\\n

We now have a problem that looks like our earlier scenario with the\\nstateful firewalls, but with NAT devices:

\\n\\n \\n \\n\\n\\n ![\"Diagram](\"https://cdn.sanity.io/images/w77i7m8x/production/20d2f883c81f9771bc15fcde173334b46f7beabb-2180x620.png\")\\n \\n\\n\\n

Our problem is that our two peers don’t know what the `ip:port` of\\ntheir peer is. Worse, strictly speaking there is *no* `ip:port` until\\nthe other peer sends packets, since NAT mappings only get created when\\noutbound traffic towards the internet requires it. We’re back to our\\nstateful firewall problem, only worse: both sides have to speak first,\\nbut neither side knows to whom to speak, and can’t know until the\\nother side speaks first.

\\n

How do we break the deadlock? That’s where STUN comes in. STUN is both\\na set of studies of the detailed behavior of NAT devices, and a\\nprotocol that aids in NAT traversal. The main thing we care about for\\nnow is the network protocol.

\\n

STUN relies on a simple observation: when you talk to a server on the\\ninternet from a NATed client, the server sees the public `ip:port`\\nthat your NAT device created for you, not your LAN `ip:port`. So, the\\nserver can *tell* you what `ip:port` it saw. That way, you know what\\ntraffic from your LAN `ip:port` looks like on the internet, you can\\ntell your peers about that mapping, and now they know where to send\\npackets! We’re back to our “simple” case of firewall traversal.

\\n

That’s fundamentally all that the STUN protocol is: your machine sends\\na “what’s my endpoint from your point of view?” request to a STUN\\nserver, and the server replies with “here’s the `ip:port` that I saw\\nyour UDP packet coming from.”

\\n\\n \\n \\n\\n\\n ![\"STUN](\"https://cdn.sanity.io/images/w77i7m8x/production/b48af5689535a521c586d6f535bac4f6a95d62b7-1840x976.png\")\\n \\n\\n\\n

(The STUN protocol has a bunch more stuff in it — there’s a way of\\nobfuscating the `ip:port` in the response to stop really broken NATs\\nfrom mangling the packet’s payload, and a whole authentication\\nmechanism that only really gets used by TURN and ICE, sibling\\nprotocols to STUN that we’ll talk about in a bit. We can ignore all of\\nthat stuff for address discovery.)

\\n

Incidentally, this is why we said in the introduction that, if you\\nwant to implement this yourself, the NAT traversal logic and your main\\nprotocol have to share a network socket. Each socket gets a different\\nmapping on the NAT device, so in order to discover your public\\n`ip:port`, you have to send and receive STUN packets from the socket\\nthat you intend to use for communication, otherwise you’ll get a\\nuseless answer.

\\n

#### How this helps ####

\\n

Given STUN as a tool, it seems like we’re close to done. Each machine\\ncan do STUN to discover the public-facing `ip:port` for its local\\nsocket, tell its peers what that is, everyone does the firewall\\ntraversal stuff, and we’re all set… Right?

\\n

Well, it’s a mixed bag. This’ll work in some cases, but not\\nothers. Generally speaking, this’ll work with most home routers, and\\nwill fail with some corporate NAT gateways. The probability of failure\\nincreases the more the NAT device’s brochure mentions that it’s a\\nsecurity device. (NATs do not enhance security in any meaningful way,\\nbut that’s a rant for another time.)

\\n

The problem is an assumption we made earlier: when the STUN server\\ntold us that we’re `2.2.2.2:4242` from its perspective, we assumed\\nthat meant that we’re `2.2.2.2:4242` from the entire internet’s\\nperspective, and that therefore anyone can reach us by talking to\\n`2.2.2.2:4242`.

\\n

As it turns out, that’s not always true. Some NAT devices behave\\nexactly in line with our assumptions. Their stateful firewall\\ncomponent still wants to see packets flowing in the right order, but\\nwe can reliably figure out the correct `ip:port` to give to our peer\\nand do our simultaneous transmission trick to get through. Those NATs\\nare great, and our combination of STUN and the simultaneous packet\\nsending will work fine with those.

\\n

(in theory, there are also NAT devices that are super relaxed, and\\ndon’t ship with stateful firewall stuff at all. In those, you don’t\\neven need simultaneous transmission, the STUN request gives you an\\ninternet `ip:port` that anyone can connect to with no further\\nceremony. If such devices do still exist, they’re increasingly rare.)

\\n

Other NAT devices are more difficult, and create a completely\\ndifferent NAT mapping for every different destination that you talk\\nto. On such a device, if we use the same socket to send to\\n`5.5.5.5:1234` and `7.7.7.7:2345`, we’ll end up with two different\\nports on 2.2.2.2, one for each destination. If you use the wrong port\\nto talk back, you don’t get through.

\\n\\n \\n \\n\\n\\n ![\"Example](\"https://cdn.sanity.io/images/w77i7m8x/production/c9edd473a0702412836a0f0efa1024b2df60a22e-2000x1076.png\")\\n \\n\\n\\n

#### Naming our NATs ####

\\n

Now that we’ve discovered that not all NAT devices behave in the same\\nway, we should talk terminology. If you’ve done anything related to\\nNAT traversal before, you might have heard of “Full Cone”, “Restricted\\nCone”, “Port-Restricted Cone” and “Symmetric” NATs. These are terms\\nthat come from early research into NAT traversal.

\\n

That terminology is honestly quite confusing. I always look up what a\\nRestricted Cone NAT is supposed to be. Empirically, I’m not alone in\\nthis, because most of the internet calls “easy” NATs Full Cone, when\\nthese days they’re much more likely to be Port-Restricted Cone.

\\n

More recent research and RFCs have come up with a much better\\ntaxonomy. First of all, they recognize that there are many more\\nvarying dimensions of behavior than the single “cone” dimension of\\nearlier research, so focusing on the cone-ness of your NAT isn’t\\nnecessarily helpful. Second, they came up with words that more plainly\\nconvey what the NAT is doing.

\\n

The “easy” and “hard” NATs above differ in a single dimension: whether\\nor not their NAT mappings depend on what the destination is. [RFC\\n4787](\"https://tools.ietf.org/html/rfc4787\") calls the easy variant “Endpoint-Independent Mapping”\\n(EIM for short), and the hard variant “Endpoint-Dependent Mapping”\\n(EDM for short). There’s a subcategory of EDM that specifies whether\\nthe mapping varies only on the destination IP, or on both the\\ndestination IP and port. For NAT traversal, the distinction doesn’t\\nmatter. Both kinds of EDM NATs are equally bad news for us.

\\n

In the grand tradition of naming things being hard,\\nendpoint-independent NATs still depend on an endpoint: each *source*\\n`ip:port` gets a different mapping, because otherwise your packets\\nwould get mixed up with someone else’s packets, and that would be\\nchaos. Strictly speaking, we should say “Destination Endpoint\\nIndependent Mapping” (DEIM?), but that’s a mouthful, and since “Source\\nEndpoint Independent Mapping” would be another way to say “broken”, we\\ndon’t specify. Endpoint always means “Destination Endpoint.”

\\n

You might be wondering how 2 kinds of endpoint dependence maps into 4\\nkinds of cone-ness. The answer is that cone-ness encompasses two\\northogonal dimensions of NAT behavior. One is NAT mapping behavior,\\nwhich we looked at above, and the other is stateful firewall\\nbehavior. Like NAT mapping behavior, the firewalls can be\\nEndpoint-Independent or a couple of variants of Endpoint-Dependent. If\\nyou throw all of these into a matrix, you can reconstruct the\\ncone-ness of a NAT from its more fundamental properties:

\\n

#### NAT Cone Types ####

\\n\\n \\n \\n \\n \\n \\n \\n \\n \\n \\n \\n \\n \\n \\n \\n \\n \\n \\n \\n \\n \\n \\n \\n \\n

|                                               |**Endpoint-Independent NAT mapping**|**Endpoint-Dependent NAT mapping (all types)**|
|-----------------------------------------------|------------------------------------|----------------------------------------------|
|       **Endpoint-Independent firewall**       |           Full Cone NAT            |                    N/A\*                     |
|**Endpoint-Dependent firewall (dest. IP only)**|        Restricted Cone NAT         |                    N/A\*                     |
|**Endpoint-Dependent firewall (dest. IP+port)**|      Port-Restricted Cone NAT      |                Symmetric NAT                 |
\\n\\n

\* can theoretically exist, but don't show up in the wild

\\n\\n

Once broken down like this, we can see that cone-ness isn’t terribly\\nuseful to us. The major distinction we care about is Symmetric versus\\nanything else — in other words, we care about whether a NAT device is\\nEIM or EDM.

\\n

While it’s neat to know exactly how your firewall behaves, we don’t\\ncare from the point of view of writing NAT traversal code. Our\\nsimultaneous transmission trick will get through all three variants of\\nfirewalls. In the wild we’re overwhelmingly dealing only with\\nIP-and-port endpoint-dependent firewalls. So, for practical code, we\\ncan simplify the table down to:

\\n\\n \\n \\n \\n \\n \\n \\n \\n \\n \\n \\n \\n \\n \\n

|                   |Endpoint-Independent NAT mapping|Endpoint-Dependent NAT mapping (dest. IP only)|
|-------------------|--------------------------------|----------------------------------------------|
|**Firewall is yes**|            Easy NAT            |                   Hard NAT                   |
\\n

If you’d like to read more about the newer taxonomies of NATs, you can\\nget the full details in RFCs [4787](\"https://tools.ietf.org/html/rfc4787\") (NAT Behavioral\\nRequirements for UDP), [5382](\"https://tools.ietf.org/html/rfc5382\") (for TCP) and [5508](\"https://tools.ietf.org/html/rfc5508\")\\n(for ICMP). And if you’re implementing a NAT device, these RFCs are\\nalso your guide to what behaviors you *should* implement, to make them\\nwell-behaved devices that play well with others and don’t generate\\ncomplaints about Halo multiplayer not working.

\\n

Back to our NAT traversal. We were doing well with STUN and firewall\\ntraversal, but these hard NATs are a big problem. It only takes one of\\nthem in the whole path to break our current traversal plans.

\\n

But wait, this post is titled “how NAT traversal works”, not “how NAT\\ntraversal doesn’t work.” So presumably, I have a trick up my sleeve to\\nget out of this, right?

\\n

#### Have you considered giving up? ####

\\n

This is a good time to have the awkward part of our chat: what happens\\nwhen we empty our entire bag of tricks, and we *still* can’t get\\nthrough? A lot of NAT traversal code out there gives up and declares\\nconnectivity impossible. That’s obviously not acceptable for us;\\nTailscale is nothing without the connectivity.

\\n

We could use a relay that both sides can talk to unimpeded, and have\\nit shuffle packets back and forth. But wait, isn’t that terrible?

\\n

Sort of. It’s certainly not as good as a direct connection, but if the\\nrelay is “near enough” to the network path your direct connection\\nwould have taken, and has enough bandwidth, the impact on your\\nconnection quality isn’t huge. There will be a bit more latency, maybe\\nless bandwidth. That’s still much better than no connection at all,\\nwhich is where we were heading.

\\n

And keep in mind that we only resort to this in cases where direct\\nconnections fail. We can still establish direct connections through a\\n*lot* of different networks. Having relays to handle the long tail\\nisn’t that bad.

\\n

Additionally, some networks can break our connectivity much more\\ndirectly than by having a difficult NAT. For example, we’ve observed\\nthat the UC Berkeley guest Wi-Fi blocks all outbound UDP except for DNS\\ntraffic. No amount of clever NAT tricks is going to get around the\\nfirewall eating your packets. So, we need some kind of reliable\\nfallback no matter what.

\\n

You could implement relays in a variety of ways. The classic way is a\\nprotocol called TURN (Traversal Using Relays around NAT). We’ll skip\\nthe protocol details, but the idea is that you authenticate yourself\\nto a TURN server on the internet, and it tells you “okay, I’ve\\nallocated `ip:port`, and will relay packets for you.” You tell your\\npeer the TURN `ip:port`, and we’re back to a completely trivial\\nclient/server communication scenario.

\\n

For Tailscale, we didn’t use TURN for our relays. It’s not a\\nparticularly pleasant protocol to work with, and unlike STUN there’s\\nno real interoperability benefit since there are no open TURN servers\\non the internet.

\\n

Instead, we created [DERP (Detoured Encrypted Routing\\nProtocol)](\"/blog/how-tailscale-works#encrypted-tcp-relays-derp\"), which is a general purpose packet relaying\\nprotocol. It runs over HTTP, which is handy on networks with strict\\noutbound rules, and relays encrypted payloads based on the\\ndestination’s public key.

\\n

As we briefly touched on earlier, we use this communication path both\\nas a data relay when NAT traversal fails (in the same role as TURN in\\nother systems) and as the side channel to help with NAT\\ntraversal. DERP is both our fallback of last resort to get\\nconnectivity, and our helper to upgrade to a peer-to-peer connection,\\nwhen that’s possible.

\\n

Now that we have a relay, in addition to the traversal tricks we’ve\\ndiscussed so far, we’re in pretty good shape. We can’t get through\\n*everything* but we can get through quite a lot, and we have a backup\\nfor when we fail. If you stopped reading now and implemented just the\\nabove, I’d estimate you could get a direct connection over 90% of the\\ntime, and your relays guarantee *some* connectivity all the time.

\\n

### NAT notes for nerds ###

\\n

But… If you’re not satisfied with “good enough”, there’s still a lot\\nmore we can do! What follows is a somewhat miscellaneous set of\\ntricks, which can help us out in specific situations. None of them\\nwill solve NAT traversal by itself, but by combining them judiciously,\\nwe can get incrementally closer to a 100% success rate.

\\n

#### The benefits of birthdays ####

\\n

Let’s revisit our problem with hard NATs. The key issue is that the\\nside with the easy NAT doesn’t know what `ip:port` to send to on the\\nhard side. But *must* send to the right `ip:port` in order to open up\\nits firewall to return traffic. What can we do about that?

\\n\\n \\n \\n\\n\\n ![\"Illustration](\"https://cdn.sanity.io/images/w77i7m8x/production/647364b5f593aafded475c9018f5a299f9893104-2000x760.png\")\\n \\n\\n\\n

Well, we know *some* `ip:port` for the hard side, because we ran\\nSTUN. Let’s assume that the IP address we got is correct. That’s not\\n*necessarily* true, but let’s run with the assumption for now. As it\\nturns out, it’s mostly safe to assume this. (If you’re curious why,\\nsee REQ-2 in [RFC 4787](\"https://tools.ietf.org/html/rfc4787\").)

\\n

If the IP address is correct, our only unknown is the port. There’s\\n65,535 possibilities… Could we try all of them? At 100 packets/sec,\\nthat’s a worst case of 10 minutes to find the right one. It’s better\\nthan nothing, but not great. And it *really* looks like a port scan\\n(because in fairness, it is), which may anger network intrusion\\ndetection software.

\\n

We can do much better than that, with the help of the [birthday\\nparadox](\"https://en.wikipedia.org/wiki/Birthday_problem\"). Rather than open 1 port on the hard side and have the\\neasy side try 65,535 possibilities, let’s open, say, 256 ports on the\\nhard side (by having 256 sockets sending to the easy side’s\\n`ip:port`), and have the easy side probe target ports at random.

\\n

I’ll spare you the detailed math, but you can check out the dinky\\n[python calculator](\"https://github.com/danderson/nat-birthday-paradox\") I made while working it out. The\\ncalculation is a very slight variant on the “classic” birthday\\nparadox, because it’s looking at collisions between two sets\\ncontaining distinct elements, rather than collisions within a single\\nset. Fortunately, the difference works out slightly in our favor!\\nHere’s the chances of a collision of open ports (i.e. successful\\ncommunication), as the number of random probes from the easy side\\nincreases, and assuming 256 ports on the hard side:

\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n\\n

|Number of random probes|Chance of success|
|-----------------------|-----------------|
|          174          |       50%       |
|          256          |       64%       |
|         1024          |       98%       |
|         2048          |      99.9%      |
\\n

If we stick with a fairly modest probing rate of 100 ports/sec, half\\nthe time we’ll get through in under 2 seconds. And even if we get\\nunlucky, 20 seconds in we’re virtually guaranteed to have found a way\\nin, after probing less than 4% of the total search space.

\\n

That’s great! With this additional trick, one hard NAT in the path is\\nan annoying speedbump, but we can manage. What about two?

\\n\\n \\n \\n\\n\\n ![\"Diagram](\"https://cdn.sanity.io/images/w77i7m8x/production/82546090404cf5ab862b7f1ed541ee13a34d2d5a-2000x760.png\")\\n \\n\\n\\n

We can try to apply the same trick, but now the search is much harder:\\neach random destination port we probe through a hard NAT also results\\nin a random *source* port. That means we’re now looking for a\\ncollision on a `{source port, destination port}` pair, rather than\\njust the destination port.

\\n

Again I’ll spare you the calculations, but after 20 seconds in the\\nsame regime as the previous setup (256 probes from one side, 2048 from\\nthe other), our chance of success is… 0.01%.

\\n

This shouldn’t be surprising if you’ve studied the birthday paradox\\nbefore. The birthday paradox lets us convert `N` “effort” into\\nsomething on the order of `sqrt(N)`. But we squared the size of the\\nsearch space, so even the reduced amount of effort is still a lot more\\neffort. To hit a 99.9% chance of success, we need each side to send\\n170,000 probes. At 100 packets/sec, that’s 28 minutes of trying before\\nwe can communicate. 50% of the time we’ll succeed after “only” 54,000\\npackets, but that’s still 9 minutes of waiting around with no\\nconnection. Still, that’s better than the 1.2 *years* it would take\\nwithout the birthday paradox.

\\n

In some applications, 28 minutes might still be worth it. Spend half\\nan hour brute-forcing your way through, then you can keep pinging to\\nkeep the open path alive indefinitely — or at least until one of the\\nNATs reboots and dumps all its state, then you’re back to brute\\nforcing. But it’s not looking good for any kind of interactive\\nconnectivity.

\\n

Worse, if you look at common office routers, you’ll find that they\\nhave a surprisingly low limit on active sessions. For example, a\\nJuniper SRX 300 maxes out at 64,000 active sessions. We’d consume its\\nentire session table with our one attempt to get through! And that’s\\nassuming the router behaves gracefully when overloaded. *And* this is\\nall to get a single connection! What if we have 20 machines doing this\\nbehind the same router? Disaster.

\\n

Still, with this trick we can make it through a slightly harder\\nnetwork topology than before. That’s a big deal, because home routers\\ntend to be easy NATs, and hard NATs tend to be office routers or cloud\\nNAT gateways. That means this trick buys us improved connectivity for\\nthe home-to-office and home-to-cloud scenarios, as well as a few\\noffice-to-cloud and cloud-to-cloud scenarios.

\\n

#### Partially manipulating port maps ####

\\n

Our hard NATs would be so much easier if we could ask the NATs to stop\\nbeing such jerks, and let more stuff through. Turns out, there’s a\\nprotocol for that! Three of them, to be precise. Let’s talk about\\nport mapping protocols.

\\n

The oldest is the [UPnP IGD](\"https://openconnectivity.org/developer/specifications/upnp-resources/upnp/internet-gateway-device-igd-v-2-0/\") (Universal Plug’n’Play Internet\\nGateway Device) protocol. It was born in the late 1990’s, and as such\\nuses a lot of very 90’s technology (XML, SOAP, multicast HTTP over UDP\\n— yes, really) and is quite hard to implement correctly and securely —\\nbut a lot of routers shipped with UPnP, and a lot still do. If we\\nstrip away all the fluff, we find a very simple request-response that\\nall three of our port mapping protocols implement: “Hi, please forward\\na WAN port to `lan-ip:port`,” and “okay, I’ve allocated `wan-ip:port`\\nfor you.”

\\n

Speaking of stripping away the fluff: some years after UPnP IGD came\\nout, Apple launched a competing protocol called [NAT-PMP](\"https://tools.ietf.org/html/rfc6886\")\\n(NAT Port Mapping Protocol). Unlike UPnP, it *only* does port\\nforwarding, and is extremely simple to implement, both on clients and\\non NAT devices. A little bit after that, NAT-PMP v2 was reborn as\\n[PCP](\"https://tools.ietf.org/html/rfc6887\") (Port Control Protocol).

\\n

So, to help our connectivity further, we can look for UPnP IGD,\\nNAT-PMP and PCP on our local default gateway. If one of the protocols\\nelicits a response, we request a public port mapping. You can think of\\nthis as a sort of supercharged STUN: in addition to discovering our\\npublic `ip:port`, we can instruct the NAT to be friendlier to our\\npeers, by not enforcing firewall rules for that port. Any packet from\\nanywhere that lands on our mapped port will make it back to us.

\\n

You can’t rely on these protocols being present. They might not be\\nimplemented on your devices. They might be disabled by default and\\nnobody knew to turn them on. They might have been disabled by policy.

\\n

Disabling by policy is fairly common because UPnP suffered from a\\nnumber of high-profile vulnerabilities (since fixed, so newer devices\\ncan safely offer UPnP, if implemented properly). Unfortunately, many\\ndevices come with a single “UPnP” checkbox that actually toggles UPnP,\\nNAT-PMP and PCP all at once, so folks concerned about UPnP’s security\\nend up disabling the perfectly safe alternatives as well.

\\n

Still, when it’s available, it effectively makes one NAT vanish from\\nthe data path, which usually makes connectivity trivial… But let’s\\nlook at the unusual cases.

\\n

#### Negotiating numerous NATs ####

\\n

So far, the topologies we’ve looked at have each client behind one NAT\\ndevice, with the two NATs facing each other. What happens if we build\\na “double NAT”, by chaining two NATs in front of one of our machines?

\\n\\n \\n \\n\\n\\n ![\"What](\"https://cdn.sanity.io/images/w77i7m8x/production/b35c4c4e9151c88c07ef9b77daf2a1bcc97dc36b-2000x760.png\")\\n \\n \\n

What happens if we build a “double NAT”, by chaining two NATs in front of one of our machines?

\\n \\n\\n\\n

In this example, not much of interest happens. Packets from client A\\ngo through two different layers of NAT on their way to the\\ninternet. But the outcome is the same as it was with multiple layers\\nof stateful firewalls: the extra layer is invisible to everyone, and\\nour other techniques will work fine regardless of how many layers\\nthere are. All that matters is the behavior of the “last” layer before\\nthe internet, because that’s the one that our peer has to find a way\\nthrough.

\\n

The big thing that breaks is our port mapping protocols. They act upon\\nthe layer of NAT closest to the client, whereas the one we need to\\ninfluence is the one furthest away. You can still use the port mapping\\nprotocols, but you’ll get an `ip:port` in the “middle” network, which\\nyour remote peer cannot reach. Unfortunately, none of the protocols\\ngive you enough information to find the “next NAT up” to repeat the\\nprocess there, although you could try your luck with a traceroute and\\nsome blind requests to the next few hops.

\\n

Breaking port mapping protocols is the reason why the internet is so\\nfull of warnings about the evils of double-NAT, and how you should\\nbend over backwards to avoid them. But in fact, double-NAT is entirely\\ninvisible to most internet-using applications, because most\\napplications don’t try to do this kind of explicit NAT traversal.

\\n

I’m definitely not saying that you *should* set up a double-NAT in\\nyour network. Breaking the port mapping protocols will degrade\\nmultiplayer on many video games, and will likely strip IPv6 from your\\nnetwork, which robs you of some very good options for NAT-free\\nconnectivity. But, if circumstances beyond your control force you into\\na double-NAT, and you can live with the downsides, most things will\\nstill work fine.

\\n

Which is a good thing, because you know what circumstances beyond your\\ncontrol force you to double-NAT? Let’s talk carrier-grade NAT.

\\n

#### Concerning CGNATs ####

\\n

Even with NATs to stretch the supply of IPv4 addresses, we’re *still*\\nrunning out, and ISPs can no longer afford to give one entire public\\nIP address to every home on their network. To work around this, ISPs\\napply SNAT recursively: your home router SNATs your devices to an\\n“intermediate” IP address, and further out in the ISP’s network a\\nsecond layer of NAT devices map those intermediate IPs onto a smaller\\nnumber of public IPs. This is “carrier-grade NAT”, or CGNAT for short.

\\n\\n \\n \\n\\n\\n ![\"Diagram](\"https://cdn.sanity.io/images/w77i7m8x/production/96c154b056a1c0a992e131423c81fcfb1e6df368-2100x1080.png\")\\n \\n\\n\\n\\n \\n \\n\\n\\n ![\"How](\"https://cdn.sanity.io/images/w77i7m8x/production/4fb6b92dd545e2e365cd5739e5ef4dbc9b554875-2100x1080.png\")\\n \\n \\n

How do we connect two peers who are behind the same CGNAT, but different home NATs within?

\\n \\n\\n\\n

Carrier-grade NAT is an important development for NAT traversal. Prior\\nto CGNAT, enterprising users could work around NAT traversal\\ndifficulties by manually configuring port forwarding on their home\\nrouters. But you can’t reconfigure the ISP’s CGNAT! Now even power\\nusers have to wrestle with the problems NATs pose.

\\n

The good news: this is a run of the mill double-NAT, and so as we\\ncovered above it’s mostly okay. Some stuff won’t work as well as it\\ncould, but things work well enough that ISPs can charge money for\\nit. Aside from the port mapping protocols, everything from our current\\nbag of tricks works fine in a CGNAT world.

\\n

We do have to overcome a new challenge, however: how do we connect two\\npeers who are behind the same CGNAT, but different home NATs within?\\nThat’s how we set up peers A and B in the diagram above.

\\n

The problem here is that STUN doesn’t work the way we’d like. We’d\\nlike to find out our `ip:port` on the “middle network”, because it’s\\neffectively playing the role of a miniature internet to our two\\npeers. But STUN tells us what our `ip:port` is from the STUN server’s\\npoint of view, and the STUN server is out on the internet, beyond the\\nCGNAT.

\\n

If you’re thinking that port mapping protocols can help us here,\\nyou’re right! If either peer’s home NAT supports one of the port\\nmapping protocols, we’re happy, because we have an `ip:port` that\\nbehaves like an un-NATed server, and connecting is\\ntrivial. Ironically, the fact that double NAT “breaks” the port\\nmapping protocols helps us! Of course, we still can’t count on these\\nprotocols helping us out, doubly so because CGNAT ISPs tend to turn\\nthem off in the equipment they put in homes in order to avoid software\\ngetting confused by the “wrong” results they would get.

\\n

But what if we don’t get lucky, and can’t map ports on our NATs? Let’s\\ngo back to our STUN-based technique and see what happens. Both peers\\nare behind the same CGNAT, so let’s say that STUN tells us that peer A\\nis `2.2.2.2:1234`, and peer B is `2.2.2.2:5678`.

\\n

The question is: what happens when peer A sends a packet to\\n`2.2.2.2:5678`? We might hope that the following takes place in the\\nCGNAT box:

\\n

\\n
* \\n

  Apply peer A’s NAT mapping, rewrite the packet to be from `2.2.2.2:1234` and\\nto `2.2.2.2:5678`.

  \\n\\n
* \\n

  Notice that `2.2.2.2:5678` matches peer B’s *incoming* NAT mapping, rewrite\\nthe packet to be from `2.2.2.2:1234` and to peer B’s private IP.

  \\n\\n
* \\n

  Send the packet on to peer B, on the “internal” interface rather than off\\ntowards the internet.

  \\n\\n

\\n

This behavior of NATs is called hairpinning, and with all this\\ndramatic buildup you won’t be surprised to learn that hairpinning\\nworks on some NATs and not others.

\\n

In fact, a great many otherwise well-behaved NAT devices don’t support\\nhairpinning, because they make assumptions like “a packet from my\\ninternal network to a non-internal IP address will always flow\\noutwards to the internet”, and so end up dropping packets as they try\\nto turn around within the router. These assumptions might even be\\nbaked into routing silicon, where it’s impossible to fix without new\\nhardware.

\\n

Hairpinning, or lack thereof, is a trait of all NATs, not just\\nCGNATs. In most cases, it doesn’t matter, because you’d expect two LAN\\ndevices to talk directly to each other rather than hairpin through\\ntheir default gateway. And it’s a pity that it usually doesn’t matter,\\nbecause that’s probably why hairpinning is commonly broken.

\\n

But once CGNAT is involved, hairpinning becomes vital to\\nconnectivity. Hairpinning lets you apply the same tricks that you use\\nfor internet connectivity, without worrying about whether you’re\\nbehind a CGNAT. If both hairpinning and port mapping protocols fail,\\nyou’re stuck with relaying.

\\n

#### Ideally IPv6, NAT64 notwithstanding ####

\\n

By this point I expect some of you are shouting at your screens that\\nthe solution to all this nonsense is IPv6. All this is happening\\nbecause we’re running out of IPv4 addresses, and we keep piling on\\nNATs to work around that. A much simpler fix would be to not have an IP\\naddress shortage, and make every device in the world reachable without\\nNATs. Which is exactly what IPv6 gets us.

\\n

And you’re right! Sort of. It’s true that in an IPv6-only world, all\\nof this becomes much simpler. Not trivial, mind you, because we’re\\nstill stuck with stateful firewalls. Your office workstation may have\\na globally reachable IPv6 address, but I’ll bet there’s still a\\ncorporate firewall enforcing “outbound connections only” between you\\nand the greater internet. And on-device firewalls are still there,\\nenforcing the same thing.

\\n

So, we still need the firewall traversal stuff from the start of the\\narticle, and a side channel so that peers can know what `ip:port` to\\ntalk to. We’ll probably also still want fallback relays that use a\\nwell-like protocol like HTTP, to get out of networks that block\\noutbound UDP. But we can get rid of STUN, the birthday paradox trick,\\nport mapping protocols, and all the hairpinning bumf. That’s much\\nnicer!

\\n

The big catch is that we currently don’t have an all-IPv6 world. We\\nhave a world that’s mostly IPv4, and [about 33% IPv6](\"https://www.google.com/intl/en/ipv6/statistics.html\"). Those\\n34% are very unevenly distributed, so a particular set of peers could\\nbe 100% IPv6, 0% IPv6, or anywhere in between.

\\n

What this means, unfortunately, is that IPv6 isn’t *yet* the solution\\nto our problems. For now, it’s just an extra tool in our connectivity\\ntoolbox. It’ll work fantastically well with some pairs of peers, and\\nnot at all for others. If we’re aiming for “connectivity no matter\\nwhat”, we have to also do IPv4+NAT stuff.

\\n

Meanwhile, the coexistence of IPv6 and IPv4 introduces yet another new\\nscenario we have to account for: NAT64 devices.

\\n\\n \\n \\n\\n\\n ![\"Diagram](\"https://cdn.sanity.io/images/w77i7m8x/production/d84e6f36bd77b3cd3b0524c7fac9a5c17fb96f05-2000x900.png\")\\n \\n\\n\\n

So far, the NATs we’ve looked at have been NAT44: they translate IPv4\\naddresses on one side to different IPv4 addresses on the other\\nside. NAT64, as you might guess, translates between protocols. IPv6 on\\nthe internal side of the NAT becomes IPv4 on the external\\nside. Combined with DNS64 to translate IPv4 DNS answers into IPv6, you\\ncan present an IPv6-only network to the end device, while still giving\\naccess to the IPv4 internet.

\\n

(Incidentally, you can extend this naming scheme indefinitely. There\\nhave been some experiments with NAT46; you could deploy NAT66 if you\\nenjoy chaos; and some RFCs use NAT444 for carrier-grade NAT.)

\\n

This works fine if you only deal in DNS names. If you connect to\\ngoogle.com, turning that into an IP address involves the DNS64\\napparatus, which lets the NAT64 get involved without you being any the\\nwiser.

\\n

But we care deeply about specific IPs and ports for our NAT and\\nfirewall traversal. What about us? If we’re lucky, our device supports\\nCLAT (Customer-side translator — from Customer XLAT). CLAT makes the\\nOS pretend that it has direct IPv4 connectivity, using NAT64 behind\\nthe scenes to make it work out. On CLAT devices, we don’t need to do\\nanything special.

\\n

CLAT is very common on mobile devices, but very uncommon on desktops,\\nlaptops and servers. On those, we have to explicitly do the work CLAT\\nwould have done: detect the existence of a NAT64+DNS64 setup, and use\\nit appropriately.

\\n

Detecting NAT64+DNS64 is easy: send a DNS request to `ipv4only.arpa.`\\nThat name resolves to known, constant IPv4 addresses, and only IPv4\\naddresses. If you get IPv6 addresses back, you know that a DNS64 did\\nsome translation to steer you to a NAT64. That lets you figure out\\nwhat the NAT64 prefix is.

\\n

From there, to talk to IPv4 addresses, send IPv6 packets to `{NAT64 prefix + IPv4 address}`. Similarly, if you receive traffic from\\n`{NAT64 prefix + IPv4 address}`, that’s IPv4 traffic. Now speak STUN\\nthrough the NAT64 to discover your public `ip:port` on the NAT64, and\\nyou’re back to the classic NAT traversal problem — albeit with a bit\\nmore work.

\\n

Fortunately for us, this is a fairly esoteric corner case. Most\\nv6-only networks today are mobile operators, and almost all phones\\nsupport CLAT. ISPs running v6-only networks deploy CLAT on the router\\nthey give you, and again you end up none the wiser. But if you want to\\nget those last few opportunities for connectivity, you’ll have to\\nexplicitly support talking to v4-only peers from a v6-only network as\\nwell.

\\n

### Integrating it all with ICE ###

\\n

We’re in the home stretch. We’ve covered stateful firewalls, simple\\nand advanced NAT tricks, IPv4 and IPv6. So, implement all the above,\\nand we’re done!

\\n

Except, how do you figure out which tricks to use for a particular\\npeer? How do you figure out if this is a simple stateful firewall\\nproblem, or if it’s time to bust out the birthday paradox, or if you\\nneed to fiddle with NAT64 by hand? Or maybe the two of you are on the\\nsame Wi-Fi network, with no firewalls and no effort required.

\\n

Early research into NAT traversal had you precisely characterize the\\npath between you and your peer, and deploy a specific set of\\nworkarounds to defeat that exact path. But as it turned out, network\\nengineers and NAT box programmers have many inventive ideas, and that\\nstops scaling very quickly. We need something that involves a bit less\\nthinking on our part.

\\n

Enter the Interactive Connectivity Establishment (ICE) protocol. Like\\nSTUN and TURN, ICE has its roots in the telephony world, and so the\\nRFC is full of SIP and SDP and signalling sessions and dialing and so\\nforth. However, if you push past that, it also specifies a stunningly\\nelegant algorithm for figuring out the best way to get a connection.

\\n

Ready? The algorithm is: try everything at once, and pick the best\\nthing that works. That’s it. Isn’t that amazing?

\\n

Let’s look at this algorithm in a bit more detail. We’re going to\\ndeviate from the ICE spec here and there, so if you’re trying to\\nimplement an interoperable ICE client, you should go read [RFC\\n8445](\"https://tools.ietf.org/html/rfc8445\") and implement that. We’ll skip all the\\ntelephony-oriented stuff to focus on the core logic, and suggest a few\\nplaces where you have more degrees of freedom than the ICE spec\\nsuggests.

\\n

To communicate with a peer, we start by gathering a list of candidate\\nendpoints for our local socket. A candidate is any `ip:port` that\\nour peer might, perhaps, be able to use in order to speak to us. We\\ndon’t need to be picky at this stage, the list should include at\\nleast:

\\n

\\n
* \\n

  IPv6 `ip:ports`

  \\n\\n
* \\n

  IPv4 LAN `ip:ports`

  \\n\\n
* \\n

  IPv4 WAN `ip:ports` discovered by STUN (possibly via a NAT64 translator)

  \\n\\n
* \\n

  IPv4 WAN `ip:port` allocated by a port mapping protocol

  \\n\\n
* \\n

  Operator-provided endpoints (e.g. for statically configured port forwards)

  \\n\\n

\\n

Then, we swap candidate lists with our peer through the side channel,\\nand start sending probe packets at each others’ endpoints. Again, at\\nthis point you don’t discriminate: if the peer provided you with 15\\nendpoints, you send “are you there?” probes to all 15 of them.

\\n

These packets are pulling double duty. Their first function is to act\\nas the packets that open up the firewalls and pierce the NATs, like\\nwe’ve been doing for this entire article. But the other is to act as a\\nhealth check. We’re exchanging (hopefully authenticated) “ping” and\\n“pong” packets, to check if a particular path works end to end.

\\n

Finally, after some time has passed, we pick the “best” (according to\\nsome heuristic) candidate path that was observed to work, and we’re\\ndone.

\\n

The beauty of this algorithm is that if your heuristic is right,\\nyou’ll always get an optimal answer. ICE has you score your candidates\\nahead of time (usually: LAN \> WAN \> WAN+NAT), but it doesn’t have to\\nbe that way. Starting with v0.100.0, Tailscale switched from a\\nhardcoded preference order to round-trip latency, which tends to\\nresult in the same LAN \> WAN \> WAN+NAT ordering. But unlike static\\nordering, we discover which “category” a path falls into organically,\\nrather than having to guess ahead of time.

\\n

The ICE spec structures the protocol as a “probe phase” followed by an\\n“okay let’s communicate” phase, but there’s no reason you *need* to\\nstrictly order them. In Tailscale, we upgrade connections on the fly\\nas we discover better paths, and all connections start out with DERP\\npreselected. That means you can use the connection immediately through\\nthe fallback path, while path discovery runs in parallel. Usually,\\nafter a few seconds, we’ll have found a better path, and your\\nconnection transparently upgrades to it.

\\n

One thing to be wary of is asymmetric paths. ICE goes to some effort\\nto ensure that both peers have picked the same network path, so that\\nthere’s definite bidirectional packet flow to keep all the NATs and\\nfirewalls open. You don’t have to go to the same effort, but you *do*\\nhave to ensure that there’s bidirectional traffic along all paths\\nyou’re using. That can be as simple as continuing to send ping/pong\\nprobes periodically.

\\n

To be really robust, you also need to detect that your currently\\nselected path has failed (say, because maintenance caused your NAT’s\\nstate to get dumped on the floor), and downgrade to another path. You\\ncan do this by continuing to probe all possible paths and keep a set\\nof “warm” fallbacks ready to go, but downgrades are rare enough that\\nit’s probably more efficient to fall all the way back to your relay of\\nlast resort, then restart path discovery.

\\n

Finally, we should mention security. Throughout this article, I’ve\\nassumed that the “upper layer” protocol you’ll be running over this\\nconnection brings its own security (QUIC has TLS certs, WireGuard has\\nits own public keys…). If that’s not the case, you absolutely need\\nto bring your own. Once you’re dynamically switching paths at runtime,\\nIP-based security becomes meaningless (not that it was worth much in\\nthe first place), and you *must* have at least end-to-end\\nauthentication.

\\n

If you have security for your upper layer, strictly speaking it’s okay\\nif your ping/pong probes are spoofable. The worst that can happen is\\nthat an attacker can persuade you to relay your traffic through\\nthem. In the presence of e2e security, that’s not a *huge* deal\\n(although obviously it depends on your threat model). But for good\\nmeasure, you might as well authenticate and encrypt the path discovery\\npackets as well. Consult your local application security engineer for\\nhow to do that safely.

\\n

### Concluding our connectivity chat ###

\\n

At last, we’re done. If you implement all of the above, you’ll have\\nstate of the art NAT traversal software that can get direct\\nconnections established in the widest possible array of\\nsituations. And you’ll have your relay network to pick up the slack\\nwhen traversal fails, as it likely will for a long tail of cases.

\\n

This is all quite complicated! It’s one of those problems that’s fun\\nto explore, but quite fiddly to get right, especially if you start\\nchasing the long tail of opportunities for just that little bit more\\nconnectivity.

\\n

The good news is that, once you’ve done it, you have something of a\\nsuperpower: you get to explore the exciting and relatively\\nunder-explored world of peer-to-peer applications. So many interesting\\nideas for decentralized software fall at the first hurdle, when it\\nturns out that talking to each other on the internet is harder than\\nexpected. But now you know how to get past that, so go build cool\\nstuff!

\\n

**Here’s a parting “TL;DR” recap:** For robust NAT traversal, you need\\nthe following ingredients:

\\n

\\n
* \\n

  A UDP-based protocol to augment

  \\n\\n
* \\n

  Direct access to a socket in your program

  \\n\\n
* \\n

  A communication side channel with your peers

  \\n\\n
* \\n

  A couple of STUN servers

  \\n\\n
* \\n

  A network of fallback relays (optional, but highly recommended)

  \\n\\n

\\n

Then, you need to:

\\n

\\n
* \\n

  Enumerate all the `ip:ports` for your socket on your directly\\nconnected interfaces

  \\n\\n
* \\n

  Query STUN servers to discover WAN `ip:ports` and the “difficulty”\\nof your NAT, if any

  \\n\\n
* \\n

  Try using the port mapping protocols to find more WAN `ip:ports`

  \\n\\n
* \\n

  Check for NAT64 and discover a WAN `ip:port` through that as well,\\nif applicable

  \\n\\n
* \\n

  Exchange all those `ip:ports` with your peer through your side\\nchannel, along with some cryptographic keys to secure everything.

  \\n\\n
* \\n

  Begin communicating with your peer through fallback relays\\n(optional, for quick connection establishment)

  \\n\\n
* \\n

  Probe all of your peer’s `ip:ports` for connectivity and if\\nnecessary/desired, also execute birthday attacks to get through\\nharder NATs

  \\n\\n
* \\n

  As you discover connectivity paths that are better than the one\\nyou’re currently using, transparently upgrade away from the previous\\npaths.

  \\n\\n
* \\n

  If the active path stops working, downgrade as needed to maintain\\nconnectivity.

  \\n\\n
* \\n

  Make sure everything is encrypted and authenticated end-to-end.

  \\n\\n

\\n "}

[Blog](/blog)|August 21, 2020

How NAT traversal works

<img alt="Branded artwork in greyscale" src="https://cdn.sanity.io/images/w77i7m8x/production/f94042c9459e89db55090dc0203b042bdc2c5937-600x315.svg?w=1200&q=75&fit=clip&auto=format" height="315" width="600" />

 .Markdown p \> code { border: none; -webkit-font-smoothing: subpixel-antialiased; -moz-osx-font-smoothing: auto; }

We covered a lot of ground in our post about [*How Tailscale
Works*](/blog/how-tailscale-works). However, we glossed over how we can get through NATs
(Network Address Translators) and connect your devices directly to
each other, no matter what’s standing between them. Let’s talk about
that now!

![NAT traversal with laptops diagram.](https://cdn.sanity.io/images/w77i7m8x/production/50eafc1638d93b9637dcee0d55967a8fa09e05c7-1700x800.png)

Let’s start with a simple problem: establishing a peer-to-peer
connection between two machines. In Tailscale’s case, we want to set
up a WireGuard® tunnel, but that doesn’t really matter. The
techniques we use are widely applicable and the work of many people
over decades. For example, [WebRTC](https://webrtc.org/) uses this bag of tricks to
send peer-to-peer audio, video and data between web browsers. VoIP
phones and some video games use similar techniques, though not always
successfully.

We’ll be discussing these techniques generically, using Tailscale and
others for examples where appropriate. Let’s say you’re making your
own protocol and that you want NAT traversal. You need two things.

First, the protocol should be based on UDP. You *can* do NAT traversal
with TCP, but it adds another layer of complexity to an already quite
complex problem, and may even require kernel customizations depending
on how deep you want to go. We’re going to focus on UDP for the rest
of this article.

If you’re reaching for TCP because you want a stream-oriented
connection when the NAT traversal is done, consider using QUIC
instead. It builds on top of UDP, so we can focus on UDP for NAT
traversal and still have a nice stream protocol at the end.

Second, you need direct control over the network socket that’s sending
and receiving network packets. As a rule, you can’t take an existing
network library and make it traverse NATs, because you have to send
and receive extra packets that aren’t part of the “main” protocol
you’re trying to speak. Some protocols tightly integrate the NAT
traversal with the rest (e.g. WebRTC). But if you’re building your
own, it’s helpful to think of NAT traversal as a separate entity that
shares a socket with your main protocol. Both run in parallel, one
enabling the other.

![NAT traversal with app diagram.](https://cdn.sanity.io/images/w77i7m8x/production/5bef4a143b33f1a5b2ce49e919dccb4e59b7569a-1600x760.png)

Direct socket access may be tough depending on your situation. One
workaround is to run a local proxy. Your protocol speaks to this
proxy, and the proxy does both NAT traversal and relaying of your
packets to the peer. This layer of indirection lets you benefit from
NAT traversal without altering your original program.

With prerequisites out of the way, let’s go through NAT traversal from
first principles. Our goal is to get UDP packets flowing
bidirectionally between two devices, so that our other protocol
(WireGuard, QUIC, WebRTC, …) can do something cool. There are two
obstacles to having this Just Work: stateful firewalls and NAT
devices.

Figuring out firewalls

Stateful firewalls are the simpler of our two problems. In fact, most
NAT devices include a stateful firewall, so we need to solve this
subset before we can tackle NATs.

There are many incarnations to consider. Some you might recognize are
the Windows Defender firewall, Ubuntu’s ufw (using iptables/nftables),
BSD’s pf (also used by macOS) and AWS’s Security Groups. They’re all
very configurable, but the most common configuration allows all
“outbound” connections and blocks all “inbound” connections. There
might be a few handpicked exceptions, such as allowing inbound SSH.

But connections and “direction” are a figment of the protocol
designer’s imagination. On the wire, every connection ends up being
bidirectional; it’s all individual packets flying back and forth. How
does the firewall know what’s inbound and what’s outbound?

That’s where the stateful part comes in. Stateful firewalls remember
what packets they’ve seen in the past and can use that knowledge when
deciding what to do with new packets that show up.

![NAT traversal diagram.](https://cdn.sanity.io/images/w77i7m8x/production/2b44043b374217d6fa2d3a138b77c171df05bdf2-1600x740.png)

For UDP, the rule is very simple: the firewall allows an inbound UDP
packet if it previously saw a matching outbound packet. For example,
if our laptop firewall sees a UDP packet leaving the laptop from
2.2.2.2:1234 to 7.7.7.7:5678, it’ll make a note that incoming
packets from 7.7.7.7:5678 to 2.2.2.2:1234 are also fine. The
trusted side of the world clearly intended to communicate with
7.7.7.7:5678, so we should let them talk back.

(As an aside, some *very* relaxed firewalls might allow traffic from
anywhere back to 2.2.2.2:1234 once 2.2.2.2:1234 has communicated
with anyone. Such firewalls make our traversal job easier, but are
increasingly rare.)

Firewall face-off

This rule for UDP traffic is only a minor problem for us, as long as
all the firewalls on the path are “facing” the same way. That’s
usually the case when you’re communicating with a server on the
internet. Our only constraint is that the machine that’s *behind* the
firewall must be the one initiating all connections. Nothing can
talk to it, unless it talks first.

![NAT traversal diagram with firewall.](https://cdn.sanity.io/images/w77i7m8x/production/e4277f239f9dbd1344451fc7d5e7da067447fd4d-2100x788.png)

This is fine, but not very interesting: we’ve reinvented client/server
communication, where the server makes itself easily reachable to
clients. In the VPN world, this leads to a hub-and-spoke topology: the
hub has no firewalls blocking access to it and the firewalled spokes
connect to the hub.

![VPN AWS diagram.](https://cdn.sanity.io/images/w77i7m8x/production/2bbe8d0fa6f3a66b71f0aec01bf831a3cc1a8a65-2210x1082.png)

The problems start when two of our “clients” want to talk
directly. Now the firewalls are facing each other. According to the
rule we established above, this means both sides must go first, but
also that neither can go first, because the other side has to go
first!

![Diagram of two 'clients' trying to talk directly but facing firewalls prevent further action.](https://cdn.sanity.io/images/w77i7m8x/production/928409c960e0b0bcd53560edf80a934b24eaec11-1740x620.png)

How do we get around this? One way would be to require users to
reconfigure one or both of the firewalls to “open a port” and allow
the other machine’s traffic. This is not very user friendly. It also
doesn’t scale to mesh networks like Tailscale, in which we expect the
peers to be moving around the internet with some regularity. And, of
course, in many cases you don’t have control over the firewalls: you
can’t reconfigure the router in your favorite coffee shop, or at the
airport. (At least, hopefully not!)

We need another option. One that doesn’t involve reconfiguring
firewalls.

Finessing finicky firewalls

The trick is to carefully read the rule we established for our
stateful firewalls. For UDP, the rule is: **packets must flow out
before packets can flow back in.**

However, nothing says the packets must be *related* to each other
beyond the IPs and ports lining up correctly. As long as *some* packet
flowed outwards with the right source and destination, any packet that*looks like* a response will be allowed back in, even if the other
side never received your packet!

So, to traverse these multiple stateful firewalls, we need to share
some information to get underway: the peers have to know in advance
the ip:port their counterpart is using. One approach is to
statically configure each peer by hand, but this approach doesn’t
scale very far. To move beyond that, we built a [coordination
server](/blog/how-tailscale-works#the-control-plane-key-exchange-and-coordination) to keep the ip:port information synchronized in a
flexible, secure manner.

Then, the peers start sending UDP packets to each other. They must
expect some of these packets to get lost, so they can’t carry any
precious information unless you’re prepared to retransmit them. This
is generally true of UDP, but especially true here. We’re *going* to
lose some packets in this process.

Our laptop and workstation are now listening on fixed ports, so that
they both know exactly what ip:port to talk to. Let’s take a look at
what happens.

![Diagram shows packet flow blocked by facing firewalls.](https://cdn.sanity.io/images/w77i7m8x/production/d7310815a3e9f715b549b2043f60b94b04b42b6d-1740x680.png)

The laptop’s first packet, from 2.2.2.2:1234 to 7.7.7.7:5678, goes
through the Windows Defender firewall and out to the internet. The
corporate firewall on the other end blocks the packet, since it has no
record of 7.7.7.7:5678 ever talking to 2.2.2.2:1234. However,
Windows Defender now remembers that it should expect and allow
responses from 7.7.7.7:5678 to 2.2.2.2:1234.

![Packet flow blocked by facing firewalls as shown in diagram.](https://cdn.sanity.io/images/w77i7m8x/production/9cfd8653ec918a72d6909e5603d65c2ca4b6e5c9-1740x680.png)

Next, the workstation’s first packet from 7.7.7.7:5678 to
2.2.2.2:1234 goes through the corporate firewall and across the
internet. When it arrives at the laptop, Windows Defender thinks “ah,
a response to that outbound request I saw”, and lets the packet
through! Additionally, the corporate firewall now remembers that it
should expect responses from 2.2.2.2:1234 to 7.7.7.7:5678, and
that those packets are also okay.

Encouraged by the receipt of a packet from the workstation, the laptop
sends another packet back. It goes through the Windows Defender
firewall, through the corporate firewall (because it’s a “response” to
a previously sent packet), and arrives at the workstation.

![NAT traversal packet exchange.](https://cdn.sanity.io/images/w77i7m8x/production/7189e9a0b2caf65998dc2e50e84e8f4a35e73bbb-1740x680.png)

Success! We’ve established two-way communication through a pair of
firewalls that, at first glance, would have prevented it.

Creative connectivity caveats

It’s not always so easy. We’re relying on some indirect influence over
third-party systems, which requires careful handling. What do we need
to keep in mind when managing firewall-traversing connections?

Both endpoints must attempt communication at roughly the same time, so
that all the intermediate firewalls open up while both peers are still
around. One approach is to have the peers retry continuously, but this
is wasteful. Wouldn’t it be better if both peers knew to start
establishing a connection at the same time?

This may sound a little recursive: to communicate, first you need to
be able to communicate. However, this preexisting “side channel”
doesn’t need to be very fancy: it can have a few seconds of latency,
and only needs to deliver a few thousand bytes in total, so a tiny VM
can easily be a matchmaker for thousands of machines.

In the distant past, I used XMPP chat messages as the side channel,
with great results. As another example, WebRTC requires you to come up
with your own “signalling channel” (a name that reveals WebRTC’s IP
telephony ancestry), and plug it into the WebRTC APIs. In Tailscale,
our coordination server and fleet of DERP (Detour Encrypted Routing
Protocol) servers act as our side channel.

Stateful firewalls have limited memory, meaning that we need periodic
communication to keep connections alive. If no packets are seen for a
while (a common value for UDP is 30 seconds), the firewall forgets
about the session, and we have to start over. To avoid this, we use a
timer and must either send packets regularly to reset the timers, or
have some out-of-band way of restarting the connection on demand.

On the plus side, one thing we *don’t* need to worry about is exactly
how many firewalls exist between our two peers. As long as they are
stateful and allow outbound connections, the simultaneous transmission
technique will get through any number of layers. That’s really nice,
because it means we get to implement the logic once, and it’ll work
everywhere.

…Right?

Well, not quite. For this to work, our peers need to know in advance
what ip:port to use for their counterparts. This is where NATs come
into play, and ruin our fun.

The nature of NATs

We can think of NAT (Network Address Translator) devices as stateful
firewalls with one more really annoying feature: in addition to all
the stateful firewalling stuff, they also alter packets as they go
through.

A NAT device is anything that does any kind of
Network Address Translation, i.e. altering the source or destination
IP address or port. However, when talking about connectivity problems
and NAT traversal, all the problems come from Source NAT, or SNAT for
short. As you might expect, there is also DNAT (Destination NAT), and
it’s very useful but not relevant to NAT traversal.

The most common use of SNAT is to connect many devices to the
internet, using fewer IP addresses than the number of devices. In the
case of consumer-grade routers, we map all devices onto a single
public-facing IP address. This is desirable because it turns out that
there are way more devices in the world that want internet access,
than IP addresses to give them (at least in IPv4 — we’ll come to IPv6
in a little bit). NATs let us have many devices sharing a single IP
address, so despite the global shortage of IPv4 addresses, we can
scale the internet further with the addresses at hand.

Navigating a NATty network

Let’s look at what happens when your laptop is connected to your home
Wi-Fi and talks to a server on the internet.

![Diagram shows established two-way communication through a pair of firewalls.](https://cdn.sanity.io/images/w77i7m8x/production/6eb196d17f6a5fc312a5ed657f6f5a2a47213cf3-2000x760.png)

Your laptop sends UDP packets from 192.168.0.20:1234 to
7.7.7.7:5678. This is exactly the same as if the laptop had a public
IP. But that won’t work on the internet: 192.168.0.20 is a private
IP address, which appears on many different peoples’ private
networks. The internet won’t know how to get responses back to us.

Enter the home router. The laptop’s packets flow through the home
router on their way to the internet, and the router sees that this is
a new session that it’s never seen before.

It knows that 192.168.0.20 won’t fly on the internet, but it can
work around that: it picks some unused UDP port on its own public IP
address — we’ll use 2.2.2.2:4242 — and creates a *NAT mapping* that
establishes an equivalence: 192.168.0.20:1234 on the LAN side is the
same as 2.2.2.2:4242 on the internet side.

From now on, whenever it sees packets that match that mapping, it will rewrite
the IPs and ports in the packet appropriately.

![Diagram shows laptop connected to home Wi-Fi, talking to a server on the internet.](https://cdn.sanity.io/images/w77i7m8x/production/953404277d143f8e1ece8df72697208593faccb0-2080x640.png)

Resuming our packet’s journey: the home router applies the NAT mapping
it just created, and sends the packet onwards to the internet. Only
now, the packet is from 2.2.2.2:4242, not 192.168.0.20:1234. It
goes on to the server, which is none the wiser. It’s communicating
with 2.2.2.2:4242, like in our previous examples sans NAT.

Responses from the server flow back the other way as you’d expect,
with the home router rewriting 2.2.2.2:4242 back to
192.168.0.20:1234. The laptop is *also* none the wiser, from its
perspective the internet magically figured out what to do with its
private IP address.

Our example here was with a home router, but the same principle
applies on corporate networks. The usual difference there is that the
NAT layer consists of multiple machines (for high availability or
capacity reasons), and they can have more than one public IP address,
so that they have more public ip:port combinations to choose from
and can sustain more active clients at once.

![Multiple NATs on a single layer allow for higher availability or capacity, but function the same as a single NAT.](https://cdn.sanity.io/images/w77i7m8x/production/cdd34c97cc748ad3a478656650dc5c3f6091dc12-2300x1076.png)

Multiple NATs on a single layer allow for higher availability or capacity, but function the same as a single NAT.

 A study in STUN

We now have a problem that looks like our earlier scenario with the
stateful firewalls, but with NAT devices:

![Diagram shows laptop’s packets flowing through the home router on their way to the internet.](https://cdn.sanity.io/images/w77i7m8x/production/20d2f883c81f9771bc15fcde173334b46f7beabb-2180x620.png)

Our problem is that our two peers don’t know what the ip:port of
their peer is. Worse, strictly speaking there is *no* ip:port until
the other peer sends packets, since NAT mappings only get created when
outbound traffic towards the internet requires it. We’re back to our
stateful firewall problem, only worse: both sides have to speak first,
but neither side knows to whom to speak, and can’t know until the
other side speaks first.

How do we break the deadlock? That’s where STUN comes in. STUN is both
a set of studies of the detailed behavior of NAT devices, and a
protocol that aids in NAT traversal. The main thing we care about for
now is the network protocol.

STUN relies on a simple observation: when you talk to a server on the
internet from a NATed client, the server sees the public ip:port
that your NAT device created for you, not your LAN ip:port. So, the
server can *tell* you what ip:port it saw. That way, you know what
traffic from your LAN ip:port looks like on the internet, you can
tell your peers about that mapping, and now they know where to send
packets! We’re back to our “simple” case of firewall traversal.

That’s fundamentally all that the STUN protocol is: your machine sends
a “what’s my endpoint from your point of view?” request to a STUN
server, and the server replies with “here’s the ip:port that I saw
your UDP packet coming from.”

![STUN is both a set of studies of the detailed behavior of NAT devices, and a protocol that aids in NAT traversal.](https://cdn.sanity.io/images/w77i7m8x/production/b48af5689535a521c586d6f535bac4f6a95d62b7-1840x976.png)

(The STUN protocol has a bunch more stuff in it — there’s a way of
obfuscating the ip:port in the response to stop really broken NATs
from mangling the packet’s payload, and a whole authentication
mechanism that only really gets used by TURN and ICE, sibling
protocols to STUN that we’ll talk about in a bit. We can ignore all of
that stuff for address discovery.)

Incidentally, this is why we said in the introduction that, if you
want to implement this yourself, the NAT traversal logic and your main
protocol have to share a network socket. Each socket gets a different
mapping on the NAT device, so in order to discover your public
ip:port, you have to send and receive STUN packets from the socket
that you intend to use for communication, otherwise you’ll get a
useless answer.

How this helps

Given STUN as a tool, it seems like we’re close to done. Each machine
can do STUN to discover the public-facing ip:port for its local
socket, tell its peers what that is, everyone does the firewall
traversal stuff, and we’re all set… Right?

Well, it’s a mixed bag. This’ll work in some cases, but not
others. Generally speaking, this’ll work with most home routers, and
will fail with some corporate NAT gateways. The probability of failure
increases the more the NAT device’s brochure mentions that it’s a
security device. (NATs do not enhance security in any meaningful way,
but that’s a rant for another time.)

The problem is an assumption we made earlier: when the STUN server
told us that we’re 2.2.2.2:4242 from its perspective, we assumed
that meant that we’re 2.2.2.2:4242 from the entire internet’s
perspective, and that therefore anyone can reach us by talking to
2.2.2.2:4242.

As it turns out, that’s not always true. Some NAT devices behave
exactly in line with our assumptions. Their stateful firewall
component still wants to see packets flowing in the right order, but
we can reliably figure out the correct ip:port to give to our peer
and do our simultaneous transmission trick to get through. Those NATs
are great, and our combination of STUN and the simultaneous packet
sending will work fine with those.

(in theory, there are also NAT devices that are super relaxed, and
don’t ship with stateful firewall stuff at all. In those, you don’t
even need simultaneous transmission, the STUN request gives you an
internet ip:port that anyone can connect to with no further
ceremony. If such devices do still exist, they’re increasingly rare.)

Other NAT devices are more difficult, and create a completely
different NAT mapping for every different destination that you talk
to. On such a device, if we use the same socket to send to
5.5.5.5:1234 and 7.7.7.7:2345, we’ll end up with two different
ports on 2.2.2.2, one for each destination. If you use the wrong port
to talk back, you don’t get through.

![Example of when NAT devices are more difficult, and create a completely different NAT mapping for every different destination.](https://cdn.sanity.io/images/w77i7m8x/production/c9edd473a0702412836a0f0efa1024b2df60a22e-2000x1076.png) Naming our NATs

Now that we’ve discovered that not all NAT devices behave in the same
way, we should talk terminology. If you’ve done anything related to
NAT traversal before, you might have heard of “Full Cone”, “Restricted
Cone”, “Port-Restricted Cone” and “Symmetric” NATs. These are terms
that come from early research into NAT traversal.

That terminology is honestly quite confusing. I always look up what a
Restricted Cone NAT is supposed to be. Empirically, I’m not alone in
this, because most of the internet calls “easy” NATs Full Cone, when
these days they’re much more likely to be Port-Restricted Cone.

More recent research and RFCs have come up with a much better
taxonomy. First of all, they recognize that there are many more
varying dimensions of behavior than the single “cone” dimension of
earlier research, so focusing on the cone-ness of your NAT isn’t
necessarily helpful. Second, they came up with words that more plainly
convey what the NAT is doing.

The “easy” and “hard” NATs above differ in a single dimension: whether
or not their NAT mappings depend on what the destination is. [RFC
4787](https://tools.ietf.org/html/rfc4787) calls the easy variant “Endpoint-Independent Mapping”
(EIM for short), and the hard variant “Endpoint-Dependent Mapping”
(EDM for short). There’s a subcategory of EDM that specifies whether
the mapping varies only on the destination IP, or on both the
destination IP and port. For NAT traversal, the distinction doesn’t
matter. Both kinds of EDM NATs are equally bad news for us.

In the grand tradition of naming things being hard,
endpoint-independent NATs still depend on an endpoint: each *source*ip:port gets a different mapping, because otherwise your packets
would get mixed up with someone else’s packets, and that would be
chaos. Strictly speaking, we should say “Destination Endpoint
Independent Mapping” (DEIM?), but that’s a mouthful, and since “Source
Endpoint Independent Mapping” would be another way to say “broken”, we
don’t specify. Endpoint always means “Destination Endpoint.”

You might be wondering how 2 kinds of endpoint dependence maps into 4
kinds of cone-ness. The answer is that cone-ness encompasses two
orthogonal dimensions of NAT behavior. One is NAT mapping behavior,
which we looked at above, and the other is stateful firewall
behavior. Like NAT mapping behavior, the firewalls can be
Endpoint-Independent or a couple of variants of Endpoint-Dependent. If
you throw all of these into a matrix, you can reconstruct the
cone-ness of a NAT from its more fundamental properties:

NAT Cone Types **Endpoint-Independent NAT mapping** **Endpoint-Dependent NAT mapping (all types)** **Endpoint-Independent firewall** Full Cone NAT N/A\* **Endpoint-Dependent firewall (dest. IP only)** Restricted Cone NAT N/A\* **Endpoint-Dependent firewall (dest. IP+port)** Port-Restricted Cone NAT Symmetric NAT

\* can theoretically exist, but don't show up in the wild

Once broken down like this, we can see that cone-ness isn’t terribly
useful to us. The major distinction we care about is Symmetric versus
anything else — in other words, we care about whether a NAT device is
EIM or EDM.

While it’s neat to know exactly how your firewall behaves, we don’t
care from the point of view of writing NAT traversal code. Our
simultaneous transmission trick will get through all three variants of
firewalls. In the wild we’re overwhelmingly dealing only with
IP-and-port endpoint-dependent firewalls. So, for practical code, we
can simplify the table down to:

 Endpoint-Independent NAT mapping Endpoint-Dependent NAT mapping (dest. IP only) **Firewall is yes** Easy NAT Hard NAT

If you’d like to read more about the newer taxonomies of NATs, you can
get the full details in RFCs [4787](https://tools.ietf.org/html/rfc4787) (NAT Behavioral
Requirements for UDP), [5382](https://tools.ietf.org/html/rfc5382) (for TCP) and [5508](https://tools.ietf.org/html/rfc5508)(for ICMP). And if you’re implementing a NAT device, these RFCs are
also your guide to what behaviors you *should* implement, to make them
well-behaved devices that play well with others and don’t generate
complaints about Halo multiplayer not working.

Back to our NAT traversal. We were doing well with STUN and firewall
traversal, but these hard NATs are a big problem. It only takes one of
them in the whole path to break our current traversal plans.

But wait, this post is titled “how NAT traversal works”, not “how NAT
traversal doesn’t work.” So presumably, I have a trick up my sleeve to
get out of this, right?

Have you considered giving up?

This is a good time to have the awkward part of our chat: what happens
when we empty our entire bag of tricks, and we *still* can’t get
through? A lot of NAT traversal code out there gives up and declares
connectivity impossible. That’s obviously not acceptable for us;
Tailscale is nothing without the connectivity.

We could use a relay that both sides can talk to unimpeded, and have
it shuffle packets back and forth. But wait, isn’t that terrible?

Sort of. It’s certainly not as good as a direct connection, but if the
relay is “near enough” to the network path your direct connection
would have taken, and has enough bandwidth, the impact on your
connection quality isn’t huge. There will be a bit more latency, maybe
less bandwidth. That’s still much better than no connection at all,
which is where we were heading.

And keep in mind that we only resort to this in cases where direct
connections fail. We can still establish direct connections through a*lot* of different networks. Having relays to handle the long tail
isn’t that bad.

Additionally, some networks can break our connectivity much more
directly than by having a difficult NAT. For example, we’ve observed
that the UC Berkeley guest Wi-Fi blocks all outbound UDP except for DNS
traffic. No amount of clever NAT tricks is going to get around the
firewall eating your packets. So, we need some kind of reliable
fallback no matter what.

You could implement relays in a variety of ways. The classic way is a
protocol called TURN (Traversal Using Relays around NAT). We’ll skip
the protocol details, but the idea is that you authenticate yourself
to a TURN server on the internet, and it tells you “okay, I’ve
allocated ip:port, and will relay packets for you.” You tell your
peer the TURN ip:port, and we’re back to a completely trivial
client/server communication scenario.

For Tailscale, we didn’t use TURN for our relays. It’s not a
particularly pleasant protocol to work with, and unlike STUN there’s
no real interoperability benefit since there are no open TURN servers
on the internet.

Instead, we created [DERP (Detoured Encrypted Routing
Protocol)](/blog/how-tailscale-works#encrypted-tcp-relays-derp), which is a general purpose packet relaying
protocol. It runs over HTTP, which is handy on networks with strict
outbound rules, and relays encrypted payloads based on the
destination’s public key.

As we briefly touched on earlier, we use this communication path both
as a data relay when NAT traversal fails (in the same role as TURN in
other systems) and as the side channel to help with NAT
traversal. DERP is both our fallback of last resort to get
connectivity, and our helper to upgrade to a peer-to-peer connection,
when that’s possible.

Now that we have a relay, in addition to the traversal tricks we’ve
discussed so far, we’re in pretty good shape. We can’t get through*everything* but we can get through quite a lot, and we have a backup
for when we fail. If you stopped reading now and implemented just the
above, I’d estimate you could get a direct connection over 90% of the
time, and your relays guarantee *some* connectivity all the time.

NAT notes for nerds

But… If you’re not satisfied with “good enough”, there’s still a lot
more we can do! What follows is a somewhat miscellaneous set of
tricks, which can help us out in specific situations. None of them
will solve NAT traversal by itself, but by combining them judiciously,
we can get incrementally closer to a 100% success rate.

The benefits of birthdays

Let’s revisit our problem with hard NATs. The key issue is that the
side with the easy NAT doesn’t know what ip:port to send to on the
hard side. But *must* send to the right ip:port in order to open up
its firewall to return traffic. What can we do about that?

![Illustration of key issue when the side with the easy NAT doesn’t know what ip:port to send to on the hard side.](https://cdn.sanity.io/images/w77i7m8x/production/647364b5f593aafded475c9018f5a299f9893104-2000x760.png)

Well, we know *some* ip:port for the hard side, because we ran
STUN. Let’s assume that the IP address we got is correct. That’s not*necessarily* true, but let’s run with the assumption for now. As it
turns out, it’s mostly safe to assume this. (If you’re curious why,
see REQ-2 in [RFC 4787](https://tools.ietf.org/html/rfc4787).)

If the IP address is correct, our only unknown is the port. There’s
65,535 possibilities… Could we try all of them? At 100 packets/sec,
that’s a worst case of 10 minutes to find the right one. It’s better
than nothing, but not great. And it *really* looks like a port scan
(because in fairness, it is), which may anger network intrusion
detection software.

We can do much better than that, with the help of the [birthday
paradox](https://en.wikipedia.org/wiki/Birthday_problem). Rather than open 1 port on the hard side and have the
easy side try 65,535 possibilities, let’s open, say, 256 ports on the
hard side (by having 256 sockets sending to the easy side’s
ip:port), and have the easy side probe target ports at random.

I’ll spare you the detailed math, but you can check out the dinky[python calculator](https://github.com/danderson/nat-birthday-paradox) I made while working it out. The
calculation is a very slight variant on the “classic” birthday
paradox, because it’s looking at collisions between two sets
containing distinct elements, rather than collisions within a single
set. Fortunately, the difference works out slightly in our favor!
Here’s the chances of a collision of open ports (i.e. successful
communication), as the number of random probes from the easy side
increases, and assuming 256 ports on the hard side:

 Number of random probes
Chance of success 174
50% 256
64% 1024
98% 2048
99.9%

If we stick with a fairly modest probing rate of 100 ports/sec, half
the time we’ll get through in under 2 seconds. And even if we get
unlucky, 20 seconds in we’re virtually guaranteed to have found a way
in, after probing less than 4% of the total search space.

That’s great! With this additional trick, one hard NAT in the path is
an annoying speedbump, but we can manage. What about two?

![Diagram shows random destination ports probed through a hard NAT resulting in a random source port.](https://cdn.sanity.io/images/w77i7m8x/production/82546090404cf5ab862b7f1ed541ee13a34d2d5a-2000x760.png)

We can try to apply the same trick, but now the search is much harder:
each random destination port we probe through a hard NAT also results
in a random *source* port. That means we’re now looking for a
collision on a {source port, destination port} pair, rather than
just the destination port.

Again I’ll spare you the calculations, but after 20 seconds in the
same regime as the previous setup (256 probes from one side, 2048 from
the other), our chance of success is… 0.01%.

This shouldn’t be surprising if you’ve studied the birthday paradox
before. The birthday paradox lets us convert N “effort” into
something on the order of sqrt(N). But we squared the size of the
search space, so even the reduced amount of effort is still a lot more
effort. To hit a 99.9% chance of success, we need each side to send
170,000 probes. At 100 packets/sec, that’s 28 minutes of trying before
we can communicate. 50% of the time we’ll succeed after “only” 54,000
packets, but that’s still 9 minutes of waiting around with no
connection. Still, that’s better than the 1.2 *years* it would take
without the birthday paradox.

In some applications, 28 minutes might still be worth it. Spend half
an hour brute-forcing your way through, then you can keep pinging to
keep the open path alive indefinitely — or at least until one of the
NATs reboots and dumps all its state, then you’re back to brute
forcing. But it’s not looking good for any kind of interactive
connectivity.

Worse, if you look at common office routers, you’ll find that they
have a surprisingly low limit on active sessions. For example, a
Juniper SRX 300 maxes out at 64,000 active sessions. We’d consume its
entire session table with our one attempt to get through! And that’s
assuming the router behaves gracefully when overloaded. *And* this is
all to get a single connection! What if we have 20 machines doing this
behind the same router? Disaster.

Still, with this trick we can make it through a slightly harder
network topology than before. That’s a big deal, because home routers
tend to be easy NATs, and hard NATs tend to be office routers or cloud
NAT gateways. That means this trick buys us improved connectivity for
the home-to-office and home-to-cloud scenarios, as well as a few
office-to-cloud and cloud-to-cloud scenarios.

Partially manipulating port maps

Our hard NATs would be so much easier if we could ask the NATs to stop
being such jerks, and let more stuff through. Turns out, there’s a
protocol for that! Three of them, to be precise. Let’s talk about
port mapping protocols.

The oldest is the [UPnP IGD](https://openconnectivity.org/developer/specifications/upnp-resources/upnp/internet-gateway-device-igd-v-2-0/) (Universal Plug’n’Play Internet
Gateway Device) protocol. It was born in the late 1990’s, and as such
uses a lot of very 90’s technology (XML, SOAP, multicast HTTP over UDP
— yes, really) and is quite hard to implement correctly and securely —
but a lot of routers shipped with UPnP, and a lot still do. If we
strip away all the fluff, we find a very simple request-response that
all three of our port mapping protocols implement: “Hi, please forward
a WAN port to lan-ip:port,” and “okay, I’ve allocated wan-ip:port
for you.”

Speaking of stripping away the fluff: some years after UPnP IGD came
out, Apple launched a competing protocol called [NAT-PMP](https://tools.ietf.org/html/rfc6886)(NAT Port Mapping Protocol). Unlike UPnP, it *only* does port
forwarding, and is extremely simple to implement, both on clients and
on NAT devices. A little bit after that, NAT-PMP v2 was reborn as[PCP](https://tools.ietf.org/html/rfc6887) (Port Control Protocol).

So, to help our connectivity further, we can look for UPnP IGD,
NAT-PMP and PCP on our local default gateway. If one of the protocols
elicits a response, we request a public port mapping. You can think of
this as a sort of supercharged STUN: in addition to discovering our
public ip:port, we can instruct the NAT to be friendlier to our
peers, by not enforcing firewall rules for that port. Any packet from
anywhere that lands on our mapped port will make it back to us.

You can’t rely on these protocols being present. They might not be
implemented on your devices. They might be disabled by default and
nobody knew to turn them on. They might have been disabled by policy.

Disabling by policy is fairly common because UPnP suffered from a
number of high-profile vulnerabilities (since fixed, so newer devices
can safely offer UPnP, if implemented properly). Unfortunately, many
devices come with a single “UPnP” checkbox that actually toggles UPnP,
NAT-PMP and PCP all at once, so folks concerned about UPnP’s security
end up disabling the perfectly safe alternatives as well.

Still, when it’s available, it effectively makes one NAT vanish from
the data path, which usually makes connectivity trivial… But let’s
look at the unusual cases.

Negotiating numerous NATs

So far, the topologies we’ve looked at have each client behind one NAT
device, with the two NATs facing each other. What happens if we build
a “double NAT”, by chaining two NATs in front of one of our machines?

![What happens if we build a “double NAT”, by chaining two NATs in front of one of our machines?](https://cdn.sanity.io/images/w77i7m8x/production/b35c4c4e9151c88c07ef9b77daf2a1bcc97dc36b-2000x760.png)

What happens if we build a “double NAT”, by chaining two NATs in front of one of our machines?

In this example, not much of interest happens. Packets from client A
go through two different layers of NAT on their way to the
internet. But the outcome is the same as it was with multiple layers
of stateful firewalls: the extra layer is invisible to everyone, and
our other techniques will work fine regardless of how many layers
there are. All that matters is the behavior of the “last” layer before
the internet, because that’s the one that our peer has to find a way
through.

The big thing that breaks is our port mapping protocols. They act upon
the layer of NAT closest to the client, whereas the one we need to
influence is the one furthest away. You can still use the port mapping
protocols, but you’ll get an ip:port in the “middle” network, which
your remote peer cannot reach. Unfortunately, none of the protocols
give you enough information to find the “next NAT up” to repeat the
process there, although you could try your luck with a traceroute and
some blind requests to the next few hops.

Breaking port mapping protocols is the reason why the internet is so
full of warnings about the evils of double-NAT, and how you should
bend over backwards to avoid them. But in fact, double-NAT is entirely
invisible to most internet-using applications, because most
applications don’t try to do this kind of explicit NAT traversal.

I’m definitely not saying that you *should* set up a double-NAT in
your network. Breaking the port mapping protocols will degrade
multiplayer on many video games, and will likely strip IPv6 from your
network, which robs you of some very good options for NAT-free
connectivity. But, if circumstances beyond your control force you into
a double-NAT, and you can live with the downsides, most things will
still work fine.

Which is a good thing, because you know what circumstances beyond your
control force you to double-NAT? Let’s talk carrier-grade NAT.

Concerning CGNATs

Even with NATs to stretch the supply of IPv4 addresses, we’re *still*running out, and ISPs can no longer afford to give one entire public
IP address to every home on their network. To work around this, ISPs
apply SNAT recursively: your home router SNATs your devices to an
“intermediate” IP address, and further out in the ISP’s network a
second layer of NAT devices map those intermediate IPs onto a smaller
number of public IPs. This is “carrier-grade NAT”, or CGNAT for short.

![Diagram of “carrier-grade NAT” (CGNAT).](https://cdn.sanity.io/images/w77i7m8x/production/96c154b056a1c0a992e131423c81fcfb1e6df368-2100x1080.png) ![How do we connect two peers who are behind the same CGNAT, but different home NATs within?](https://cdn.sanity.io/images/w77i7m8x/production/4fb6b92dd545e2e365cd5739e5ef4dbc9b554875-2100x1080.png)

How do we connect two peers who are behind the same CGNAT, but different home NATs within?

Carrier-grade NAT is an important development for NAT traversal. Prior
to CGNAT, enterprising users could work around NAT traversal
difficulties by manually configuring port forwarding on their home
routers. But you can’t reconfigure the ISP’s CGNAT! Now even power
users have to wrestle with the problems NATs pose.

The good news: this is a run of the mill double-NAT, and so as we
covered above it’s mostly okay. Some stuff won’t work as well as it
could, but things work well enough that ISPs can charge money for
it. Aside from the port mapping protocols, everything from our current
bag of tricks works fine in a CGNAT world.

We do have to overcome a new challenge, however: how do we connect two
peers who are behind the same CGNAT, but different home NATs within?
That’s how we set up peers A and B in the diagram above.

The problem here is that STUN doesn’t work the way we’d like. We’d
like to find out our ip:port on the “middle network”, because it’s
effectively playing the role of a miniature internet to our two
peers. But STUN tells us what our ip:port is from the STUN server’s
point of view, and the STUN server is out on the internet, beyond the
CGNAT.

If you’re thinking that port mapping protocols can help us here,
you’re right! If either peer’s home NAT supports one of the port
mapping protocols, we’re happy, because we have an ip:port that
behaves like an un-NATed server, and connecting is
trivial. Ironically, the fact that double NAT “breaks” the port
mapping protocols helps us! Of course, we still can’t count on these
protocols helping us out, doubly so because CGNAT ISPs tend to turn
them off in the equipment they put in homes in order to avoid software
getting confused by the “wrong” results they would get.

But what if we don’t get lucky, and can’t map ports on our NATs? Let’s
go back to our STUN-based technique and see what happens. Both peers
are behind the same CGNAT, so let’s say that STUN tells us that peer A
is 2.2.2.2:1234, and peer B is 2.2.2.2:5678.

The question is: what happens when peer A sends a packet to
2.2.2.2:5678? We might hope that the following takes place in the
CGNAT box:

* Apply peer A’s NAT mapping, rewrite the packet to be from 2.2.2.2:1234 and
  to 2.2.2.2:5678.

* Notice that 2.2.2.2:5678 matches peer B’s *incoming* NAT mapping, rewrite
  the packet to be from 2.2.2.2:1234 and to peer B’s private IP.

* Send the packet on to peer B, on the “internal” interface rather than off
  towards the internet.

This behavior of NATs is called hairpinning, and with all this
dramatic buildup you won’t be surprised to learn that hairpinning
works on some NATs and not others.

In fact, a great many otherwise well-behaved NAT devices don’t support
hairpinning, because they make assumptions like “a packet from my
internal network to a non-internal IP address will always flow
outwards to the internet”, and so end up dropping packets as they try
to turn around within the router. These assumptions might even be
baked into routing silicon, where it’s impossible to fix without new
hardware.

Hairpinning, or lack thereof, is a trait of all NATs, not just
CGNATs. In most cases, it doesn’t matter, because you’d expect two LAN
devices to talk directly to each other rather than hairpin through
their default gateway. And it’s a pity that it usually doesn’t matter,
because that’s probably why hairpinning is commonly broken.

But once CGNAT is involved, hairpinning becomes vital to
connectivity. Hairpinning lets you apply the same tricks that you use
for internet connectivity, without worrying about whether you’re
behind a CGNAT. If both hairpinning and port mapping protocols fail,
you’re stuck with relaying.

Ideally IPv6, NAT64 notwithstanding

By this point I expect some of you are shouting at your screens that
the solution to all this nonsense is IPv6. All this is happening
because we’re running out of IPv4 addresses, and we keep piling on
NATs to work around that. A much simpler fix would be to not have an IP
address shortage, and make every device in the world reachable without
NATs. Which is exactly what IPv6 gets us.

And you’re right! Sort of. It’s true that in an IPv6-only world, all
of this becomes much simpler. Not trivial, mind you, because we’re
still stuck with stateful firewalls. Your office workstation may have
a globally reachable IPv6 address, but I’ll bet there’s still a
corporate firewall enforcing “outbound connections only” between you
and the greater internet. And on-device firewalls are still there,
enforcing the same thing.

So, we still need the firewall traversal stuff from the start of the
article, and a side channel so that peers can know what ip:port to
talk to. We’ll probably also still want fallback relays that use a
well-like protocol like HTTP, to get out of networks that block
outbound UDP. But we can get rid of STUN, the birthday paradox trick,
port mapping protocols, and all the hairpinning bumf. That’s much
nicer!

The big catch is that we currently don’t have an all-IPv6 world. We
have a world that’s mostly IPv4, and [about 33% IPv6](https://www.google.com/intl/en/ipv6/statistics.html). Those
34% are very unevenly distributed, so a particular set of peers could
be 100% IPv6, 0% IPv6, or anywhere in between.

What this means, unfortunately, is that IPv6 isn’t *yet* the solution
to our problems. For now, it’s just an extra tool in our connectivity
toolbox. It’ll work fantastically well with some pairs of peers, and
not at all for others. If we’re aiming for “connectivity no matter
what”, we have to also do IPv4+NAT stuff.

Meanwhile, the coexistence of IPv6 and IPv4 introduces yet another new
scenario we have to account for: NAT64 devices.

![Diagram shows the coexistence of IPv6 and IPv4 introducing a scenario for NAT64 devices.](https://cdn.sanity.io/images/w77i7m8x/production/d84e6f36bd77b3cd3b0524c7fac9a5c17fb96f05-2000x900.png)

So far, the NATs we’ve looked at have been NAT44: they translate IPv4
addresses on one side to different IPv4 addresses on the other
side. NAT64, as you might guess, translates between protocols. IPv6 on
the internal side of the NAT becomes IPv4 on the external
side. Combined with DNS64 to translate IPv4 DNS answers into IPv6, you
can present an IPv6-only network to the end device, while still giving
access to the IPv4 internet.

(Incidentally, you can extend this naming scheme indefinitely. There
have been some experiments with NAT46; you could deploy NAT66 if you
enjoy chaos; and some RFCs use NAT444 for carrier-grade NAT.)

This works fine if you only deal in DNS names. If you connect to
google.com, turning that into an IP address involves the DNS64
apparatus, which lets the NAT64 get involved without you being any the
wiser.

But we care deeply about specific IPs and ports for our NAT and
firewall traversal. What about us? If we’re lucky, our device supports
CLAT (Customer-side translator — from Customer XLAT). CLAT makes the
OS pretend that it has direct IPv4 connectivity, using NAT64 behind
the scenes to make it work out. On CLAT devices, we don’t need to do
anything special.

CLAT is very common on mobile devices, but very uncommon on desktops,
laptops and servers. On those, we have to explicitly do the work CLAT
would have done: detect the existence of a NAT64+DNS64 setup, and use
it appropriately.

Detecting NAT64+DNS64 is easy: send a DNS request to ipv4only.arpa.
That name resolves to known, constant IPv4 addresses, and only IPv4
addresses. If you get IPv6 addresses back, you know that a DNS64 did
some translation to steer you to a NAT64. That lets you figure out
what the NAT64 prefix is.

From there, to talk to IPv4 addresses, send IPv6 packets to {NAT64 prefix + IPv4 address}. Similarly, if you receive traffic from
{NAT64 prefix + IPv4 address}, that’s IPv4 traffic. Now speak STUN
through the NAT64 to discover your public ip:port on the NAT64, and
you’re back to the classic NAT traversal problem — albeit with a bit
more work.

Fortunately for us, this is a fairly esoteric corner case. Most
v6-only networks today are mobile operators, and almost all phones
support CLAT. ISPs running v6-only networks deploy CLAT on the router
they give you, and again you end up none the wiser. But if you want to
get those last few opportunities for connectivity, you’ll have to
explicitly support talking to v4-only peers from a v6-only network as
well.

Integrating it all with ICE

We’re in the home stretch. We’ve covered stateful firewalls, simple
and advanced NAT tricks, IPv4 and IPv6. So, implement all the above,
and we’re done!

Except, how do you figure out which tricks to use for a particular
peer? How do you figure out if this is a simple stateful firewall
problem, or if it’s time to bust out the birthday paradox, or if you
need to fiddle with NAT64 by hand? Or maybe the two of you are on the
same Wi-Fi network, with no firewalls and no effort required.

Early research into NAT traversal had you precisely characterize the
path between you and your peer, and deploy a specific set of
workarounds to defeat that exact path. But as it turned out, network
engineers and NAT box programmers have many inventive ideas, and that
stops scaling very quickly. We need something that involves a bit less
thinking on our part.

Enter the Interactive Connectivity Establishment (ICE) protocol. Like
STUN and TURN, ICE has its roots in the telephony world, and so the
RFC is full of SIP and SDP and signalling sessions and dialing and so
forth. However, if you push past that, it also specifies a stunningly
elegant algorithm for figuring out the best way to get a connection.

Ready? The algorithm is: try everything at once, and pick the best
thing that works. That’s it. Isn’t that amazing?

Let’s look at this algorithm in a bit more detail. We’re going to
deviate from the ICE spec here and there, so if you’re trying to
implement an interoperable ICE client, you should go read [RFC
8445](https://tools.ietf.org/html/rfc8445) and implement that. We’ll skip all the
telephony-oriented stuff to focus on the core logic, and suggest a few
places where you have more degrees of freedom than the ICE spec
suggests.

To communicate with a peer, we start by gathering a list of candidate
endpoints for our local socket. A candidate is any ip:port that
our peer might, perhaps, be able to use in order to speak to us. We
don’t need to be picky at this stage, the list should include at
least:

* IPv6 ip:ports

* IPv4 LAN ip:ports

* IPv4 WAN ip:ports discovered by STUN (possibly via a NAT64 translator)

* IPv4 WAN ip:port allocated by a port mapping protocol

* Operator-provided endpoints (e.g. for statically configured port forwards)

Then, we swap candidate lists with our peer through the side channel,
and start sending probe packets at each others’ endpoints. Again, at
this point you don’t discriminate: if the peer provided you with 15
endpoints, you send “are you there?” probes to all 15 of them.

These packets are pulling double duty. Their first function is to act
as the packets that open up the firewalls and pierce the NATs, like
we’ve been doing for this entire article. But the other is to act as a
health check. We’re exchanging (hopefully authenticated) “ping” and
“pong” packets, to check if a particular path works end to end.

Finally, after some time has passed, we pick the “best” (according to
some heuristic) candidate path that was observed to work, and we’re
done.

The beauty of this algorithm is that if your heuristic is right,
you’ll always get an optimal answer. ICE has you score your candidates
ahead of time (usually: LAN \> WAN \> WAN+NAT), but it doesn’t have to
be that way. Starting with v0.100.0, Tailscale switched from a
hardcoded preference order to round-trip latency, which tends to
result in the same LAN \> WAN \> WAN+NAT ordering. But unlike static
ordering, we discover which “category” a path falls into organically,
rather than having to guess ahead of time.

The ICE spec structures the protocol as a “probe phase” followed by an
“okay let’s communicate” phase, but there’s no reason you *need* to
strictly order them. In Tailscale, we upgrade connections on the fly
as we discover better paths, and all connections start out with DERP
preselected. That means you can use the connection immediately through
the fallback path, while path discovery runs in parallel. Usually,
after a few seconds, we’ll have found a better path, and your
connection transparently upgrades to it.

One thing to be wary of is asymmetric paths. ICE goes to some effort
to ensure that both peers have picked the same network path, so that
there’s definite bidirectional packet flow to keep all the NATs and
firewalls open. You don’t have to go to the same effort, but you *do*have to ensure that there’s bidirectional traffic along all paths
you’re using. That can be as simple as continuing to send ping/pong
probes periodically.

To be really robust, you also need to detect that your currently
selected path has failed (say, because maintenance caused your NAT’s
state to get dumped on the floor), and downgrade to another path. You
can do this by continuing to probe all possible paths and keep a set
of “warm” fallbacks ready to go, but downgrades are rare enough that
it’s probably more efficient to fall all the way back to your relay of
last resort, then restart path discovery.

Finally, we should mention security. Throughout this article, I’ve
assumed that the “upper layer” protocol you’ll be running over this
connection brings its own security (QUIC has TLS certs, WireGuard has
its own public keys…). If that’s not the case, you absolutely need
to bring your own. Once you’re dynamically switching paths at runtime,
IP-based security becomes meaningless (not that it was worth much in
the first place), and you *must* have at least end-to-end
authentication.

If you have security for your upper layer, strictly speaking it’s okay
if your ping/pong probes are spoofable. The worst that can happen is
that an attacker can persuade you to relay your traffic through
them. In the presence of e2e security, that’s not a *huge* deal
(although obviously it depends on your threat model). But for good
measure, you might as well authenticate and encrypt the path discovery
packets as well. Consult your local application security engineer for
how to do that safely.

Concluding our connectivity chat

At last, we’re done. If you implement all of the above, you’ll have
state of the art NAT traversal software that can get direct
connections established in the widest possible array of
situations. And you’ll have your relay network to pick up the slack
when traversal fails, as it likely will for a long tail of cases.

This is all quite complicated! It’s one of those problems that’s fun
to explore, but quite fiddly to get right, especially if you start
chasing the long tail of opportunities for just that little bit more
connectivity.

The good news is that, once you’ve done it, you have something of a
superpower: you get to explore the exciting and relatively
under-explored world of peer-to-peer applications. So many interesting
ideas for decentralized software fall at the first hurdle, when it
turns out that talking to each other on the internet is harder than
expected. But now you know how to get past that, so go build cool
stuff!

**Here’s a parting “TL;DR” recap:** For robust NAT traversal, you need
the following ingredients:

* A UDP-based protocol to augment

* Direct access to a socket in your program

* A communication side channel with your peers

* A couple of STUN servers

* A network of fallback relays (optional, but highly recommended)

Then, you need to:

* Enumerate all the ip:ports for your socket on your directly
  connected interfaces

* Query STUN servers to discover WAN ip:ports and the “difficulty”
  of your NAT, if any

* Try using the port mapping protocols to find more WAN ip:ports

* Check for NAT64 and discover a WAN ip:port through that as well,
  if applicable

* Exchange all those ip:ports with your peer through your side
  channel, along with some cryptographic keys to secure everything.

* Begin communicating with your peer through fallback relays
  (optional, for quick connection establishment)

* Probe all of your peer’s ip:ports for connectivity and if
  necessary/desired, also execute birthday attacks to get through
  harder NATs

* As you discover connectivity paths that are better than the one
  you’re currently using, transparently upgrade away from the previous
  paths.

* If the active path stops working, downgrade as needed to maintain
  connectivity.

* Make sure everything is encrypted and authenticated end-to-end.

Share

Author

<img alt="David Anderson" src="https://cdn.sanity.io/images/w77i7m8x/production/c1d565287da262060d5300fef5cbce3953d78617-1200x1197.jpg?w=3840&q=75&fit=clip&auto=format" height="1197" width="1200" />David Anderson

Author

<img alt="David Anderson" src="https://cdn.sanity.io/images/w77i7m8x/production/c1d565287da262060d5300fef5cbce3953d78617-1200x1197.jpg?w=3840&q=75&fit=clip&auto=format" height="1197" width="1200" />David Anderson

Share

Loading...

Try Tailscale for free

[

Get started

](https://login.tailscale.com/start)

Schedule a demo

[

Contact sales

](/contact/sales)

<img alt="cta phone" src="https://cdn.sanity.io/images/w77i7m8x/production/b715b4ca5e2577da60f0d529a4a9bc2ad4cadf59-362x567.svg?w=750&q=75&fit=clip&auto=format" height="567" width="362" />

<img alt="mercury" src="https://cdn.sanity.io/images/w77i7m8x/production/459a7a8492910eeb22f22bb8d4c0f864b0bae25f-199x81.svg?w=640&q=75&fit=clip&auto=format" height="81" width="199" />

<img alt="instacrt" src="https://cdn.sanity.io/images/w77i7m8x/production/7d127f4bb62a408b056328349f291857df6251b3-199x81.svg?w=640&q=75&fit=clip&auto=format" height="81" width="199" />

<img alt="Retool" src="https://cdn.sanity.io/images/w77i7m8x/production/e9579b00087d7896e9cb750f4eb39f2c11ed11b8-199x82.svg?w=640&q=75&fit=clip&auto=format" height="82" width="199" />

<img alt="duolingo" src="https://cdn.sanity.io/images/w77i7m8x/production/7958bf3d43a30e661ca74cf0510f250d9b99ecef-199x81.svg?w=640&q=75&fit=clip&auto=format" height="81" width="199" />

<img alt="Hugging Face" src="https://cdn.sanity.io/images/w77i7m8x/production/68e2e5024898bcd6f6d142e0306dc7564787e1d7-199x82.svg?w=640&q=75&fit=clip&auto=format" height="82" width="199" />

Product

[How it works](/blog/how-tailscale-works)[Pricing](/pricing)[Integrations](/integrations)[Features](/features)[Compare Tailscale](/compare)

Use Cases

[Business VPN](/use-cases/business-vpn)[Remote Access](/use-cases/remote-access)[Site-to-Site Networking](/use-cases/site-to-site-networking)[Homelab](/use-cases/homelab)[Enterprise](/enterprise)

Resources

[Blog](/blog)[Events & Webinars](/events-webinars)[Partnerships](/partnerships)

Company

[Company](/company)[Careers](/careers)[Press](/press)

Help & Support

[Support](/contact/support)[Sales](/contact/sales)[Security](/security)[Legal](/legal)[Open Source](/opensource)[Changelog](/changelog)

Learn

[SSH keys](/learn/generate-ssh-keys)[Docker SSH](/learn/ssh-into-docker-container)[DevSecOps](/learn/devsecops)[Multicloud](/learn/multicloud)[NAT Traversal](/blog/how-nat-traversal-works)[MagicDNS](/blog/2021-09-private-dns-with-magicdns)[PAM](/learn/privileged-access-management)[PoLP](/learn/principle-of-least-privilege)[All articles](/learn)

[](/)

[Terms of Service](/terms)[Privacy Policy](/privacy-policy)

WireGuard is a registered trademark of Jason A. Donenfeld.

[](https://twitter.com/tailscale)[](https://www.facebook.com/tailscale/)[](https://www.linkedin.com/company/tailscale)[](https://hachyderm.io/@tailscale)[](https://www.youtube.com/@Tailscale)

© 2025 Tailscale Inc. All rights reserved. Tailscale is a registered trademark of Tailscale Inc.

{"props":{"pageProps":{"global":{"announcement":{"link":{"label":"Make the switch to Tailscale","url":"https://tailscale.com/switch"},"text":"Say goodbye to your legacy VPN"},"header":{"\_createdAt":"2023-10-06T12:21:23Z","\_type":"header","menu":[{"hasSubmenu":true,"\_key":"95381f81d527","title":"Product","submenu":{"product":{"rightCol":{"nav":{"heading":"Explore","links":[{"link":"/integrations","\_key":"c653da519dfb","title":"Integrations"},{"title":"Features","link":"/features","\_key":"a878da5fa54c"},{"link":"/compare","\_key":"adda698ed879","title":"Compare Tailscale"},{"link":"/partnerships","\_key":"b57369965809","title":"Partnerships"}]}},"leftCol":{"topNav":{"heading":"Meet Tailscale","links":[{"\_key":"5495d201056a","title":"How it works","icon":{"\_type":"sanityImage","alt":"icon"},"link":"/blog/how-tailscale-works/"},{"title":"Why Tailscale","icon":{"\_type":"sanityImage","alt":"icon"},"link":"/why-tailscale","\_key":"dc9cde7ff83cb94cfc98ff29bdcd0997"},{"icon":{"\_type":"sanityImage","alt":"WireGuard®"},"link":"/wireguard-vpn","\_key":"5d88e3ffcc6b","title":"WireGuard® for Enterprises"},{"title":"Bring Tailscale to Work","icon":{"\_type":"sanityImage","alt":"Bring Tailscale to Work"},"link":"/bring-tailscale-to-work","\_key":"435de37ddd5f"}]}}},"submenuType":"product"}},{"submenu":{"product":{"rightCol":{"nav":{"heading":"By role","links":[{"title":"DevOps","link":"/solutions/devops","\_key":"502a00f49baf"},{"\_key":"0fe4c0d6fa83","title":"IT","link":"/solutions/it"},{"\_key":"026f30b876a7","title":"Security","link":"/solutions/security"}]}},"leftCol":{"topNav":{"heading":"By use-case","links":[{"title":"Remote Access","link":"/use-cases/remote-access","\_key":"193eaaa0cef8"},{"link":"/use-cases/site-to-site-networking","\_key":"05cadfcf3e65b04708a9d88060f68f9e","title":"Site-to-site Networking"},{"title":"Multi-Cloud Networking","link":"/use-cases/multi-cloud-networking","\_key":"fbd28dffeac0"},{"link":"/use-cases/kubernetes","\_key":"da202f1d966a","title":"Kubernetes Networking"},{"link":"/use-cases/iot","\_key":"8c78e633c6b1","title":"Edge \\u0026 IoT Deployments"},{"link":"/use-cases/zero-trust-networking","\_key":"6a363d694952","title":"Zero Trust Networking"},{"\_key":"9c49b97d6b06","title":"AI Workloads","link":"/use-cases/ai"},{"link":"/use-cases/secure-saas","\_key":"2602b548bd52","title":"Secure SaaS"},{"link":"/use-cases/business-vpn","\_key":"6fc65e9fe1c6","title":"Business VPN"},{"\_key":"d99d14013ab3","title":"Homelab","link":"/use-cases/homelab"}]}}},"submenuType":"product"},"hasSubmenu":true,"\_key":"a7062f1924df","title":"Solutions"},{"link":"/enterprise","hasSubmenu":false,"\_key":"fd055b16290df04c6012d0d33c2fad13","title":"Enterprise","submenu":{"submenuType":"product"}},{"submenu":{"product":{"leftCol":{"topNav":{"heading":"Nav heading here","links":[{"icon":{"asset":{"\_ref":"image-a06dc612b1e3e4f4df53a72030002600639a8738-300x120-png","\_type":"reference"},"\_type":"sanityImage","alt":"Alt text "},"link":"https://tailscale.com/customers","description":"How Cribl Enables Secure Work From Anywhere with Tailscale","\_key":"2d22491d8262","title":"Title here"}]}}},"resources":{"topNav":[{"heading":"Cribl","description":"How Cribl Enables Secure Work From Anywhere with Tailscale","\_key":"61d0f0cb130e"},{"heading":"Cribl","description":"How Cribl Enables Secure Work From Anywhere with Tailscale","\_key":"712684d509a6a6ea07ab9401bdb23f8f"},{"heading":"Cribl","description":"How Cribl Enables Secure Work From Anywhere with Tailscale","\_key":"ceac3f234a3a6923a671af91772b7e8b"}]},"submenuType":"product"},"link":"/customers","hasSubmenu":false,"\_key":"b595975539c7407a7ed4510edd549223","title":"Customers"},{"hasSubmenu":false,"\_key":"f06fabeb084c","title":"Docs","submenu":{"submenuType":"product"},"link":"/kb/1017/install/"},{"title":"Blog","submenu":{"submenuType":"product"},"link":"/blog","hasSubmenu":false,"\_key":"f2537b6fa068"},{"submenu":{"submenuType":"product"},"link":"/pricing","hasSubmenu":false,"\_key":"e1b7b44dc091","title":"Pricing"}],"button":{"buttonOptions":{"color":"black"},"\_type":"button","link":{"title":"Get started - it's free!","url":"https://login.tailscale.com/start"}},"\_rev":"DctwwV37cP5NCi4QxlfSYJ","links":[{"\_key":"157b4ad1150d","title":"Download","url":"/download"},{"\_key":"f00209e74f6b","title":"Log in","url":"https://login.tailscale.com/welcome"}],"\_id":"7797109d-2dc4-4a75-b5a3-b1019c33212f","title":"Production Header","\_updatedAt":"2025-01-09T18:38:01Z"},"footer":{"\_createdAt":"2023-10-06T14:44:29Z","\_type":"footer","footerNav":[{"\_key":"05f3fa61c972","heading":"Product","links":[{"\_key":"30386cf08177","title":"How it works","url":"/blog/how-tailscale-works/"},{"\_key":"45dec9531713","title":"Pricing","url":"/pricing"},{"\_key":"e6f4d8daff21","title":"Integrations","url":"/integrations"},{"\_key":"d4f7875a767f","title":"Features","url":"/features"},{"\_key":"64846fcdaf3b","title":"Compare Tailscale","url":"/compare"}]},{"links":[{"\_key":"7b4858603fc7","title":"Business VPN","url":"/use-cases/business-vpn"},{"title":"Remote Access","url":"/use-cases/remote-access","\_key":"06fbf46e9354"},{"\_key":"ab3e69241df2","title":"Site-to-Site Networking","url":"/use-cases/site-to-site-networking"},{"\_key":"b79f544a8266","title":"Homelab","url":"/use-cases/homelab"},{"\_key":"8660f39ec574","title":"Enterprise","url":"/enterprise"}],"\_key":"7870d03d9802","heading":"Use Cases"},{"heading":"Resources","links":[{"\_key":"b5ad8866742c","title":"Blog","url":"/blog"},{"\_key":"21869f26f11b","title":"Events \\u0026 Webinars","url":"/events-webinars"},{"\_key":"c844ea072844","title":"Partnerships","url":"/partnerships"}],"\_key":"2e262725243d"},{"heading":"Company","links":[{"\_key":"8cc3fedb5b31","title":"Company","url":"/company"},{"\_key":"e69d139c2c7c","title":"Careers","url":"/careers"},{"\_key":"ad370d7ab2c1","title":"Press","url":"/press"}],"\_key":"a1e16018d519"},{"heading":"Help \\u0026 Support","links":[{"\_key":"f7d6ef6a99c6","title":"Support","url":"/contact/support"},{"url":"/contact/sales","\_key":"18077954da8f455140153a58c74e53ba","title":"Sales"},{"\_key":"3b91a6bb3d6b","title":"Security","url":"/security"},{"\_key":"9d3e837341e2","title":"Legal","url":"/legal"},{"\_key":"a69304fe5b80","title":"Open Source","url":"/opensource"},{"\_key":"a02943ca7fdd","title":"Changelog","url":"/changelog"}],"\_key":"b25bd2c7203e"},{"heading":"Learn","links":[{"\_key":"6c45141fcc65","title":"SSH keys","url":"/learn/generate-ssh-keys/"},{"\_key":"86c070f995c4","title":"Docker SSH","url":"/learn/ssh-into-docker-container/"},{"\_key":"19c70bbf9478","title":"DevSecOps","url":"/learn/devsecops/"},{"title":"Multicloud","url":"/learn/multicloud/","\_key":"927093698579"},{"\_key":"22e6d051e763","title":"NAT Traversal","url":"/blog/how-nat-traversal-works/"},{"\_key":"4e51a8a4f0a7","title":"MagicDNS","url":"/blog/2021-09-private-dns-with-magicdns/"},{"title":"PAM","url":"/learn/privileged-access-management/","\_key":"f8f8893085b3"},{"\_key":"8775c2b1f419","title":"PoLP","url":"/learn/principle-of-least-privilege/"},{"\_key":"e7fdb19bd312","title":"All articles","url":"/learn"}],"\_key":"0bdaf34fbe61"}],"\_id":"422b4abf-6e3f-4213-ab94-a03dd444be3d","title":"Production Footer","copyrightContent":"Tailscale Inc. All rights reserved. Tailscale is a registered trademark of Tailscale Inc.","legalContent":"WireGuard is a registered trademark of Jason A. Donenfeld.","\_rev":"IlIq0JCGGmrvSiDdugDJgM","legalNav":[{"\_key":"b3b1d8dfddea","title":"Terms of Service","url":"/terms"},{"\_key":"3e22b7802445","title":"Privacy Policy","url":"/privacy-policy"}],"\_updatedAt":"2024-11-01T16:35:35Z","cta":{"secondaryCta":{"heading":"Schedule a demo","link":{"title":"Contact sales","url":"/contact/sales"}},"heading":"Try Tailscale for |free|","textCard":{"heading":"Try Tailscale for |free|","\_type":"textCard","options":{"headingFontSize":"h3","headingFontColor":"white","sectionAlignment":"left","contentFontColor":"white","headingMaxWidth":292,"hasMobileTextAlignment":false,"highlightColor":"blue-3","headingMarginBottom":"30"},"links":[{"textLink":{"\_type":"textLink","textLinkOptions":{"arrowColor":"black","underlineColor":"black"}},"button":{"buttonOptions":{"color":"white"},"\_type":"button","link":{"title":"Get started","url":"https://login.tailscale.com/start"}},"\_key":"dc04d805f7e0","type":"button"}]},"asset":{"image":{"\_type":"sanityImage","alt":"cta phone","asset":{"\_ref":"image-b715b4ca5e2577da60f0d529a4a9bc2ad4cadf59-362x567-svg","\_type":"reference"}},"\_type":"asset","type":"image"},"darkLogoGrid":[{"logo":{"alt":"mercury","asset":{"\_ref":"image-a1fb7441ec6ea5254d0f14119dbe0abf5c822f9f-199x81-svg","\_type":"reference"},"\_type":"sanityImage"},"\_key":"fb360d1bc6c4"},{"logo":{"\_type":"sanityImage","alt":"instacart","asset":{"\_ref":"image-62410277e3cd5df52c9b59e787ae52a5a2699580-199x81-svg","\_type":"reference"}},"\_key":"a24139987731"},{"logo":{"alt":"Retool","asset":{"\_ref":"image-80654c9d97220caec3e35ba29d3e7439a03d482a-199x82-svg","\_type":"reference"},"\_type":"sanityImage"},"\_key":"0fa57e2eebee"},{"logo":{"\_type":"sanityImage","alt":"duolingo","asset":{"\_type":"reference","\_ref":"image-9b799915a326b1b78decc95e6ce251b87111f2bf-199x81-svg"}},"\_key":"b47251ca28bd"},{"logo":{"\_type":"sanityImage","alt":"Hugging Face","asset":{"\_ref":"image-b36780abd0594e34b52e74176a6b61811bbed602-199x82-svg","\_type":"reference"}},"\_key":"7bf06e2e5305"}],"ctaButton":{"link":{"title":"Get started","url":"/get-started"},"buttonOptions":{"color":"white"},"\_type":"button"},"logoGrid":[{"logo":{"\_type":"sanityImage","alt":"mercury","asset":{"\_ref":"image-459a7a8492910eeb22f22bb8d4c0f864b0bae25f-199x81-svg","\_type":"reference"}},"\_key":"a3a9b2012378"},{"logo":{"\_type":"sanityImage","alt":"instacrt","asset":{"\_ref":"image-7d127f4bb62a408b056328349f291857df6251b3-199x81-svg","\_type":"reference"}},"\_key":"993b75d39e13"},{"logo":{"\_type":"sanityImage","alt":"Retool","asset":{"\_ref":"image-e9579b00087d7896e9cb750f4eb39f2c11ed11b8-199x82-svg","\_type":"reference"}},"\_key":"8449f10eb5c7"},{"logo":{"\_type":"sanityImage","alt":"duolingo","asset":{"\_ref":"image-7958bf3d43a30e661ca74cf0510f250d9b99ecef-199x81-svg","\_type":"reference"}},"\_key":"3ab303288a39"},{"logo":{"\_type":"sanityImage","alt":"Hugging Face","asset":{"\_ref":"image-68e2e5024898bcd6f6d142e0306dc7564787e1d7-199x82-svg","\_type":"reference"}},"\_key":"5e630c781c8a"}]}},"socials":null,"legal":null,"redirects":[{"destination":"/wireguard-vpn","source":"/wireguard","\_key":"8b0a3ebcf822"},{"source":"/compare/openvpn-1","\_key":"a5d36ab00f17","destination":"/compare/openvpn"}],"seo":{"ogImage":{"\_type":"image","asset":{"\_ref":"image-8e0455b2d9b33c6151016afdf2ea81d7623c2f04-1200x628-png","\_type":"reference"}}},"newsBar":null,"globalOptions":null},"preview":false,"hubspotForms":{},"post":{"publishedAt":"August 21, 2020","flexContent":null,"summary":null,"title":"How NAT traversal works","seo":{"seoDescription":"Learn how Tailscale can get through Network Address Translators (NAT) and securely connect your devices directly to each other.","indexable":true,"seoTitle":"How Network Address Translator (NAT) works | Tailscale","seoImage":{"\_type":"image","asset":{"\_ref":"image-bbf71d57c972243cea664219fc80dcfc301252c6-1200x600-png","\_type":"reference"}},"seoCanonicalURL":"https://tailscale.com/blog/how-nat-traversal-works"},"category":null,"contributors":null,"\_id":"5e485110-dd6b-4155-9f39-5f6deec3a67b","slug":{"current":"how-nat-traversal-works","\_type":"slug"},"content":"\\n \\u003cstyle\\u003e\\n .Markdown p \\u003e code {\\n border: none;\\n -webkit-font-smoothing: subpixel-antialiased;\\n -moz-osx-font-smoothing: auto;\\n }\\n\\u003c/style\\u003e\\n\\u003cp\\u003eWe covered a lot of ground in our post about \\u003ca href=\\"/blog/how-tailscale-works\\"\\u003e\\u003cem\\u003eHow Tailscale\\nWorks\\u003c/em\\u003e\\u003c/a\\u003e. However, we glossed over how we can get through NATs\\n(Network Address Translators) and connect your devices directly to\\neach other, no matter what’s standing between them. Let’s talk about\\nthat now!\\u003c/p\\u003e\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/50eafc1638d93b9637dcee0d55967a8fa09e05c7-1700x800.png\\" alt=\\"NAT traversal with laptops diagram.\\" \\u003e\\n \\n\\u003c/figure\\u003e\\n\\n\\u003cp\\u003eLet’s start with a simple problem: establishing a peer-to-peer\\nconnection between two machines. In Tailscale’s case, we want to set\\nup a WireGuard® tunnel, but that doesn’t really matter. The\\ntechniques we use are widely applicable and the work of many people\\nover decades. For example, \\u003ca href=\\"https://webrtc.org/\\" target=\\"\_blank\\" rel=\\"noreferer noopener\\"\\u003eWebRTC\\u003c/a\\u003e uses this bag of tricks to\\nsend peer-to-peer audio, video and data between web browsers. VoIP\\nphones and some video games use similar techniques, though not always\\nsuccessfully.\\u003c/p\\u003e\\n\\u003cp\\u003eWe’ll be discussing these techniques generically, using Tailscale and\\nothers for examples where appropriate. Let’s say you’re making your\\nown protocol and that you want NAT traversal. You need two things.\\u003c/p\\u003e\\n\\u003cp\\u003eFirst, the protocol should be based on UDP. You \\u003cem\\u003ecan\\u003c/em\\u003e do NAT traversal\\nwith TCP, but it adds another layer of complexity to an already quite\\ncomplex problem, and may even require kernel customizations depending\\non how deep you want to go. We’re going to focus on UDP for the rest\\nof this article.\\u003c/p\\u003e\\n\\u003cp\\u003eIf you’re reaching for TCP because you want a stream-oriented\\nconnection when the NAT traversal is done, consider using QUIC\\ninstead. It builds on top of UDP, so we can focus on UDP for NAT\\ntraversal and still have a nice stream protocol at the end.\\u003c/p\\u003e\\n\\u003cp\\u003eSecond, you need direct control over the network socket that’s sending\\nand receiving network packets. As a rule, you can’t take an existing\\nnetwork library and make it traverse NATs, because you have to send\\nand receive extra packets that aren’t part of the “main” protocol\\nyou’re trying to speak. Some protocols tightly integrate the NAT\\ntraversal with the rest (e.g. WebRTC). But if you’re building your\\nown, it’s helpful to think of NAT traversal as a separate entity that\\nshares a socket with your main protocol. Both run in parallel, one\\nenabling the other.\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/5bef4a143b33f1a5b2ce49e919dccb4e59b7569a-1600x760.png\\" alt=\\"NAT traversal with app diagram.\\" \\u003e\\n \\n\\u003c/figure\\u003e\\n\\n\\u003cp\\u003eDirect socket access may be tough depending on your situation. One\\nworkaround is to run a local proxy. Your protocol speaks to this\\nproxy, and the proxy does both NAT traversal and relaying of your\\npackets to the peer. This layer of indirection lets you benefit from\\nNAT traversal without altering your original program.\\u003c/p\\u003e\\n\\u003cp\\u003eWith prerequisites out of the way, let’s go through NAT traversal from\\nfirst principles. Our goal is to get UDP packets flowing\\nbidirectionally between two devices, so that our other protocol\\n(WireGuard, QUIC, WebRTC, …) can do something cool. There are two\\nobstacles to having this Just Work: stateful firewalls and NAT\\ndevices.\\u003c/p\\u003e\\n\\u003ch3 id=\\"figuring-out-firewalls\\"\\u003eFiguring out firewalls\\u003c/h3\\u003e\\n\\u003cp\\u003eStateful firewalls are the simpler of our two problems. In fact, most\\nNAT devices include a stateful firewall, so we need to solve this\\nsubset before we can tackle NATs.\\u003c/p\\u003e\\n\\u003cp\\u003eThere are many incarnations to consider. Some you might recognize are\\nthe Windows Defender firewall, Ubuntu’s ufw (using iptables/nftables),\\nBSD’s pf (also used by macOS) and AWS’s Security Groups. They’re all\\nvery configurable, but the most common configuration allows all\\n“outbound” connections and blocks all “inbound” connections. There\\nmight be a few handpicked exceptions, such as allowing inbound SSH.\\u003c/p\\u003e\\n\\u003cp\\u003eBut connections and “direction” are a figment of the protocol\\ndesigner’s imagination. On the wire, every connection ends up being\\nbidirectional; it’s all individual packets flying back and forth. How\\ndoes the firewall know what’s inbound and what’s outbound?\\u003c/p\\u003e\\n\\u003cp\\u003eThat’s where the stateful part comes in. Stateful firewalls remember\\nwhat packets they’ve seen in the past and can use that knowledge when\\ndeciding what to do with new packets that show up.\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/2b44043b374217d6fa2d3a138b77c171df05bdf2-1600x740.png\\" alt=\\"NAT traversal diagram.\\"\\u003e\\n \\n\\u003c/figure\\u003e\\n\\n\\u003cp\\u003eFor UDP, the rule is very simple: the firewall allows an inbound UDP\\npacket if it previously saw a matching outbound packet. For example,\\nif our laptop firewall sees a UDP packet leaving the laptop from\\n\\u003ccode\\u003e2.2.2.2:1234\\u003c/code\\u003e to \\u003ccode\\u003e7.7.7.7:5678\\u003c/code\\u003e, it’ll make a note that incoming\\npackets from \\u003ccode\\u003e7.7.7.7:5678\\u003c/code\\u003e to \\u003ccode\\u003e2.2.2.2:1234\\u003c/code\\u003e are also fine. The\\ntrusted side of the world clearly intended to communicate with\\n\\u003ccode\\u003e7.7.7.7:5678\\u003c/code\\u003e, so we should let them talk back.\\u003c/p\\u003e\\n\\u003cp\\u003e(As an aside, some \\u003cem\\u003every\\u003c/em\\u003e relaxed firewalls might allow traffic from\\nanywhere back to \\u003ccode\\u003e2.2.2.2:1234\\u003c/code\\u003e once \\u003ccode\\u003e2.2.2.2:1234\\u003c/code\\u003e has communicated\\nwith anyone. Such firewalls make our traversal job easier, but are\\nincreasingly rare.)\\u003c/p\\u003e\\n\\u003ch4 id=\\"firewall-face-off\\"\\u003eFirewall face-off\\u003c/h4\\u003e\\n\\u003cp\\u003eThis rule for UDP traffic is only a minor problem for us, as long as\\nall the firewalls on the path are “facing” the same way. That’s\\nusually the case when you’re communicating with a server on the\\ninternet. Our only constraint is that the machine that’s \\u003cem\\u003ebehind\\u003c/em\\u003e the\\nfirewall must be the one initiating all connections. Nothing can\\ntalk to it, unless it talks first.\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/e4277f239f9dbd1344451fc7d5e7da067447fd4d-2100x788.png\\" alt=\\"NAT traversal diagram with firewall.\\"\\u003e\\n \\n\\u003c/figure\\u003e\\n\\n\\u003cp\\u003eThis is fine, but not very interesting: we’ve reinvented client/server\\ncommunication, where the server makes itself easily reachable to\\nclients. In the VPN world, this leads to a hub-and-spoke topology: the\\nhub has no firewalls blocking access to it and the firewalled spokes\\nconnect to the hub.\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/2bbe8d0fa6f3a66b71f0aec01bf831a3cc1a8a65-2210x1082.png\\" alt=\\"VPN AWS diagram.\\"\\u003e\\n \\n\\u003c/figure\\u003e\\n\\n\\u003cp\\u003eThe problems start when two of our “clients” want to talk\\ndirectly. Now the firewalls are facing each other. According to the\\nrule we established above, this means both sides must go first, but\\nalso that neither can go first, because the other side has to go\\nfirst!\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/928409c960e0b0bcd53560edf80a934b24eaec11-1740x620.png\\" alt= \\"Diagram of two 'clients' trying to talk directly but facing firewalls prevent further action.\\"\\u003e\\n \\n\\u003c/figure\\u003e\\n\\n\\u003cp\\u003eHow do we get around this? One way would be to require users to\\nreconfigure one or both of the firewalls to “open a port” and allow\\nthe other machine’s traffic. This is not very user friendly. It also\\ndoesn’t scale to mesh networks like Tailscale, in which we expect the\\npeers to be moving around the internet with some regularity. And, of\\ncourse, in many cases you don’t have control over the firewalls: you\\ncan’t reconfigure the router in your favorite coffee shop, or at the\\nairport. (At least, hopefully not!)\\u003c/p\\u003e\\n\\u003cp\\u003eWe need another option. One that doesn’t involve reconfiguring\\nfirewalls.\\u003c/p\\u003e\\n\\u003ch4 id=\\"finessing-finicky-firewalls\\"\\u003eFinessing finicky firewalls\\u003c/h4\\u003e\\n\\u003cp\\u003eThe trick is to carefully read the rule we established for our\\nstateful firewalls. For UDP, the rule is: \\u003cstrong\\u003epackets must flow out\\nbefore packets can flow back in.\\u003c/strong\\u003e\\u003c/p\\u003e\\n\\u003cp\\u003eHowever, nothing says the packets must be \\u003cem\\u003erelated\\u003c/em\\u003e to each other\\nbeyond the IPs and ports lining up correctly. As long as \\u003cem\\u003esome\\u003c/em\\u003e packet\\nflowed outwards with the right source and destination, any packet that\\n\\u003cem\\u003elooks like\\u003c/em\\u003e a response will be allowed back in, even if the other\\nside never received your packet!\\u003c/p\\u003e\\n\\u003cp\\u003eSo, to traverse these multiple stateful firewalls, we need to share\\nsome information to get underway: the peers have to know in advance\\nthe \\u003ccode\\u003eip:port\\u003c/code\\u003e their counterpart is using. One approach is to\\nstatically configure each peer by hand, but this approach doesn’t\\nscale very far. To move beyond that, we built a \\u003ca href=\\"/blog/how-tailscale-works#the-control-plane-key-exchange-and-coordination\\"\\u003ecoordination\\nserver\\u003c/a\\u003e to keep the \\u003ccode\\u003eip:port\\u003c/code\\u003e information synchronized in a\\nflexible, secure manner.\\u003c/p\\u003e\\n\\u003cp\\u003eThen, the peers start sending UDP packets to each other. They must\\nexpect some of these packets to get lost, so they can’t carry any\\nprecious information unless you’re prepared to retransmit them. This\\nis generally true of UDP, but especially true here. We’re \\u003cem\\u003egoing\\u003c/em\\u003e to\\nlose some packets in this process.\\u003c/p\\u003e\\n\\u003cp\\u003eOur laptop and workstation are now listening on fixed ports, so that\\nthey both know exactly what \\u003ccode\\u003eip:port\\u003c/code\\u003e to talk to. Let’s take a look at\\nwhat happens.\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/d7310815a3e9f715b549b2043f60b94b04b42b6d-1740x680.png\\" alt= \\"Diagram shows packet flow blocked by facing firewalls.\\"\\u003e\\n \\n\\u003c/figure\\u003e\\n\\n\\u003cp\\u003eThe laptop’s first packet, from \\u003ccode\\u003e2.2.2.2:1234\\u003c/code\\u003e to \\u003ccode\\u003e7.7.7.7:5678\\u003c/code\\u003e, goes\\nthrough the Windows Defender firewall and out to the internet. The\\ncorporate firewall on the other end blocks the packet, since it has no\\nrecord of \\u003ccode\\u003e7.7.7.7:5678\\u003c/code\\u003e ever talking to \\u003ccode\\u003e2.2.2.2:1234\\u003c/code\\u003e. However,\\nWindows Defender now remembers that it should expect and allow\\nresponses from \\u003ccode\\u003e7.7.7.7:5678\\u003c/code\\u003e to \\u003ccode\\u003e2.2.2.2:1234\\u003c/code\\u003e.\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/9cfd8653ec918a72d6909e5603d65c2ca4b6e5c9-1740x680.png\\" alt=\\"Packet flow blocked by facing firewalls as shown in diagram.\\"\\u003e\\n \\n\\u003c/figure\\u003e\\n\\n\\u003cp\\u003eNext, the workstation’s first packet from \\u003ccode\\u003e7.7.7.7:5678\\u003c/code\\u003e to\\n\\u003ccode\\u003e2.2.2.2:1234\\u003c/code\\u003e goes through the corporate firewall and across the\\ninternet. When it arrives at the laptop, Windows Defender thinks “ah,\\na response to that outbound request I saw”, and lets the packet\\nthrough! Additionally, the corporate firewall now remembers that it\\nshould expect responses from \\u003ccode\\u003e2.2.2.2:1234\\u003c/code\\u003e to \\u003ccode\\u003e7.7.7.7:5678\\u003c/code\\u003e, and\\nthat those packets are also okay.\\u003c/p\\u003e\\n\\u003cp\\u003eEncouraged by the receipt of a packet from the workstation, the laptop\\nsends another packet back. It goes through the Windows Defender\\nfirewall, through the corporate firewall (because it’s a “response” to\\na previously sent packet), and arrives at the workstation.\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/7189e9a0b2caf65998dc2e50e84e8f4a35e73bbb-1740x680.png\\" alt=\\"NAT traversal packet exchange.\\"\\u003e\\n \\n\\u003c/figure\\u003e\\n\\n\\u003cp\\u003eSuccess! We’ve established two-way communication through a pair of\\nfirewalls that, at first glance, would have prevented it.\\u003c/p\\u003e\\n\\u003ch4 id=\\"creative-connectivity-caveats\\"\\u003eCreative connectivity caveats\\u003c/h4\\u003e\\n\\u003cp\\u003eIt’s not always so easy. We’re relying on some indirect influence over\\nthird-party systems, which requires careful handling. What do we need\\nto keep in mind when managing firewall-traversing connections?\\u003c/p\\u003e\\n\\u003cp\\u003eBoth endpoints must attempt communication at roughly the same time, so\\nthat all the intermediate firewalls open up while both peers are still\\naround. One approach is to have the peers retry continuously, but this\\nis wasteful. Wouldn’t it be better if both peers knew to start\\nestablishing a connection at the same time?\\u003c/p\\u003e\\n\\u003cp\\u003eThis may sound a little recursive: to communicate, first you need to\\nbe able to communicate. However, this preexisting “side channel”\\ndoesn’t need to be very fancy: it can have a few seconds of latency,\\nand only needs to deliver a few thousand bytes in total, so a tiny VM\\ncan easily be a matchmaker for thousands of machines.\\u003c/p\\u003e\\n\\u003cp\\u003eIn the distant past, I used XMPP chat messages as the side channel,\\nwith great results. As another example, WebRTC requires you to come up\\nwith your own “signalling channel” (a name that reveals WebRTC’s IP\\ntelephony ancestry), and plug it into the WebRTC APIs. In Tailscale,\\nour coordination server and fleet of DERP (Detour Encrypted Routing\\nProtocol) servers act as our side channel.\\u003c/p\\u003e\\n\\u003cp\\u003eStateful firewalls have limited memory, meaning that we need periodic\\ncommunication to keep connections alive. If no packets are seen for a\\nwhile (a common value for UDP is 30 seconds), the firewall forgets\\nabout the session, and we have to start over. To avoid this, we use a\\ntimer and must either send packets regularly to reset the timers, or\\nhave some out-of-band way of restarting the connection on demand.\\u003c/p\\u003e\\n\\u003cp\\u003eOn the plus side, one thing we \\u003cem\\u003edon’t\\u003c/em\\u003e need to worry about is exactly\\nhow many firewalls exist between our two peers. As long as they are\\nstateful and allow outbound connections, the simultaneous transmission\\ntechnique will get through any number of layers. That’s really nice,\\nbecause it means we get to implement the logic once, and it’ll work\\neverywhere.\\u003c/p\\u003e\\n\\u003cp\\u003e…Right?\\u003c/p\\u003e\\n\\u003cp\\u003eWell, not quite. For this to work, our peers need to know in advance\\nwhat \\u003ccode\\u003eip:port\\u003c/code\\u003e to use for their counterparts. This is where NATs come\\ninto play, and ruin our fun.\\u003c/p\\u003e\\n\\u003ch3 id=\\"the-nature-of-nats\\"\\u003eThe nature of NATs\\u003c/h3\\u003e\\n\\u003cp\\u003eWe can think of NAT (Network Address Translator) devices as stateful\\nfirewalls with one more really annoying feature: in addition to all\\nthe stateful firewalling stuff, they also alter packets as they go\\nthrough.\\u003c/p\\u003e\\n\\u003cp\\u003eA NAT device is anything that does any kind of\\nNetwork Address Translation, i.e. altering the source or destination\\nIP address or port. However, when talking about connectivity problems\\nand NAT traversal, all the problems come from Source NAT, or SNAT for\\nshort. As you might expect, there is also DNAT (Destination NAT), and\\nit’s very useful but not relevant to NAT traversal.\\u003c/p\\u003e\\n\\u003cp\\u003eThe most common use of SNAT is to connect many devices to the\\ninternet, using fewer IP addresses than the number of devices. In the\\ncase of consumer-grade routers, we map all devices onto a single\\npublic-facing IP address. This is desirable because it turns out that\\nthere are way more devices in the world that want internet access,\\nthan IP addresses to give them (at least in IPv4 — we’ll come to IPv6\\nin a little bit). NATs let us have many devices sharing a single IP\\naddress, so despite the global shortage of IPv4 addresses, we can\\nscale the internet further with the addresses at hand.\\u003c/p\\u003e\\n\\u003ch4 id=\\"navigating-a-natty-network\\"\\u003eNavigating a NATty network\\u003c/h4\\u003e\\n\\u003cp\\u003eLet’s look at what happens when your laptop is connected to your home\\nWi-Fi and talks to a server on the internet.\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/6eb196d17f6a5fc312a5ed657f6f5a2a47213cf3-2000x760.png\\" alt=\\"Diagram shows established two-way communication through a pair of firewalls.\\"\\u003e\\n \\n\\u003c/figure\\u003e\\n\\n\\u003cp\\u003eYour laptop sends UDP packets from \\u003ccode\\u003e192.168.0.20:1234\\u003c/code\\u003e to\\n\\u003ccode\\u003e7.7.7.7:5678\\u003c/code\\u003e. This is exactly the same as if the laptop had a public\\nIP. But that won’t work on the internet: \\u003ccode\\u003e192.168.0.20\\u003c/code\\u003e is a private\\nIP address, which appears on many different peoples’ private\\nnetworks. The internet won’t know how to get responses back to us.\\u003c/p\\u003e\\n\\u003cp\\u003eEnter the home router. The laptop’s packets flow through the home\\nrouter on their way to the internet, and the router sees that this is\\na new session that it’s never seen before.\\u003c/p\\u003e\\n\\u003cp\\u003eIt knows that \\u003ccode\\u003e192.168.0.20\\u003c/code\\u003e won’t fly on the internet, but it can\\nwork around that: it picks some unused UDP port on its own public IP\\naddress — we’ll use \\u003ccode\\u003e2.2.2.2:4242\\u003c/code\\u003e — and creates a \\u003cem\\u003eNAT mapping\\u003c/em\\u003e that\\nestablishes an equivalence: \\u003ccode\\u003e192.168.0.20:1234\\u003c/code\\u003e on the LAN side is the\\nsame as \\u003ccode\\u003e2.2.2.2:4242\\u003c/code\\u003e on the internet side.\\u003c/p\\u003e\\n\\u003cp\\u003eFrom now on, whenever it sees packets that match that mapping, it will rewrite\\nthe IPs and ports in the packet appropriately.\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/953404277d143f8e1ece8df72697208593faccb0-2080x640.png\\" alt= \\"Diagram shows laptop connected to home Wi-Fi, talking to a server on the internet.\\"\\u003e\\n \\n\\u003c/figure\\u003e\\n\\n\\u003cp\\u003eResuming our packet’s journey: the home router applies the NAT mapping\\nit just created, and sends the packet onwards to the internet. Only\\nnow, the packet is from \\u003ccode\\u003e2.2.2.2:4242\\u003c/code\\u003e, not \\u003ccode\\u003e192.168.0.20:1234\\u003c/code\\u003e. It\\ngoes on to the server, which is none the wiser. It’s communicating\\nwith \\u003ccode\\u003e2.2.2.2:4242\\u003c/code\\u003e, like in our previous examples sans NAT.\\u003c/p\\u003e\\n\\u003cp\\u003eResponses from the server flow back the other way as you’d expect,\\nwith the home router rewriting \\u003ccode\\u003e2.2.2.2:4242\\u003c/code\\u003e back to\\n\\u003ccode\\u003e192.168.0.20:1234\\u003c/code\\u003e. The laptop is \\u003cem\\u003ealso\\u003c/em\\u003e none the wiser, from its\\nperspective the internet magically figured out what to do with its\\nprivate IP address.\\u003c/p\\u003e\\n\\u003cp\\u003eOur example here was with a home router, but the same principle\\napplies on corporate networks. The usual difference there is that the\\nNAT layer consists of multiple machines (for high availability or\\ncapacity reasons), and they can have more than one public IP address,\\nso that they have more public \\u003ccode\\u003eip:port\\u003c/code\\u003e combinations to choose from\\nand can sustain more active clients at once.\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/cdd34c97cc748ad3a478656650dc5c3f6091dc12-2300x1076.png\\" alt=\\"Multiple NATs on a single layer allow for higher availability or capacity, but function the same as a single NAT.\\"\\u003e\\n \\n \\u003cfigcaption\\u003e\\n \\u003cp\\u003eMultiple NATs on a single layer allow for higher availability or capacity, but function the same as a single NAT.\\u003c/p\\u003e\\n \\u003c/figcaption\\u003e\\n\\u003c/figure\\u003e\\n\\n\\u003ch4 id=\\"a-study-in-stun\\"\\u003eA study in STUN\\u003c/h4\\u003e\\n\\u003cp\\u003eWe now have a problem that looks like our earlier scenario with the\\nstateful firewalls, but with NAT devices:\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/20d2f883c81f9771bc15fcde173334b46f7beabb-2180x620.png\\" alt= \\"Diagram shows laptop’s packets flowing through the home router on their way to the internet.\\"\\u003e\\n \\n\\u003c/figure\\u003e\\n\\n\\u003cp\\u003eOur problem is that our two peers don’t know what the \\u003ccode\\u003eip:port\\u003c/code\\u003e of\\ntheir peer is. Worse, strictly speaking there is \\u003cem\\u003eno\\u003c/em\\u003e \\u003ccode\\u003eip:port\\u003c/code\\u003e until\\nthe other peer sends packets, since NAT mappings only get created when\\noutbound traffic towards the internet requires it. We’re back to our\\nstateful firewall problem, only worse: both sides have to speak first,\\nbut neither side knows to whom to speak, and can’t know until the\\nother side speaks first.\\u003c/p\\u003e\\n\\u003cp\\u003eHow do we break the deadlock? That’s where STUN comes in. STUN is both\\na set of studies of the detailed behavior of NAT devices, and a\\nprotocol that aids in NAT traversal. The main thing we care about for\\nnow is the network protocol.\\u003c/p\\u003e\\n\\u003cp\\u003eSTUN relies on a simple observation: when you talk to a server on the\\ninternet from a NATed client, the server sees the public \\u003ccode\\u003eip:port\\u003c/code\\u003e\\nthat your NAT device created for you, not your LAN \\u003ccode\\u003eip:port\\u003c/code\\u003e. So, the\\nserver can \\u003cem\\u003etell\\u003c/em\\u003e you what \\u003ccode\\u003eip:port\\u003c/code\\u003e it saw. That way, you know what\\ntraffic from your LAN \\u003ccode\\u003eip:port\\u003c/code\\u003e looks like on the internet, you can\\ntell your peers about that mapping, and now they know where to send\\npackets! We’re back to our “simple” case of firewall traversal.\\u003c/p\\u003e\\n\\u003cp\\u003eThat’s fundamentally all that the STUN protocol is: your machine sends\\na “what’s my endpoint from your point of view?” request to a STUN\\nserver, and the server replies with “here’s the \\u003ccode\\u003eip:port\\u003c/code\\u003e that I saw\\nyour UDP packet coming from.”\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/b48af5689535a521c586d6f535bac4f6a95d62b7-1840x976.png\\" alt= \\"STUN is both a set of studies of the detailed behavior of NAT devices, and a protocol that aids in NAT traversal.\\" \\u003e\\n \\n\\u003c/figure\\u003e\\n\\n\\u003cp\\u003e(The STUN protocol has a bunch more stuff in it — there’s a way of\\nobfuscating the \\u003ccode\\u003eip:port\\u003c/code\\u003e in the response to stop really broken NATs\\nfrom mangling the packet’s payload, and a whole authentication\\nmechanism that only really gets used by TURN and ICE, sibling\\nprotocols to STUN that we’ll talk about in a bit. We can ignore all of\\nthat stuff for address discovery.)\\u003c/p\\u003e\\n\\u003cp\\u003eIncidentally, this is why we said in the introduction that, if you\\nwant to implement this yourself, the NAT traversal logic and your main\\nprotocol have to share a network socket. Each socket gets a different\\nmapping on the NAT device, so in order to discover your public\\n\\u003ccode\\u003eip:port\\u003c/code\\u003e, you have to send and receive STUN packets from the socket\\nthat you intend to use for communication, otherwise you’ll get a\\nuseless answer.\\u003c/p\\u003e\\n\\u003ch4 id=\\"how-this-helps\\"\\u003eHow this helps\\u003c/h4\\u003e\\n\\u003cp\\u003eGiven STUN as a tool, it seems like we’re close to done. Each machine\\ncan do STUN to discover the public-facing \\u003ccode\\u003eip:port\\u003c/code\\u003e for its local\\nsocket, tell its peers what that is, everyone does the firewall\\ntraversal stuff, and we’re all set… Right?\\u003c/p\\u003e\\n\\u003cp\\u003eWell, it’s a mixed bag. This’ll work in some cases, but not\\nothers. Generally speaking, this’ll work with most home routers, and\\nwill fail with some corporate NAT gateways. The probability of failure\\nincreases the more the NAT device’s brochure mentions that it’s a\\nsecurity device. (NATs do not enhance security in any meaningful way,\\nbut that’s a rant for another time.)\\u003c/p\\u003e\\n\\u003cp\\u003eThe problem is an assumption we made earlier: when the STUN server\\ntold us that we’re \\u003ccode\\u003e2.2.2.2:4242\\u003c/code\\u003e from its perspective, we assumed\\nthat meant that we’re \\u003ccode\\u003e2.2.2.2:4242\\u003c/code\\u003e from the entire internet’s\\nperspective, and that therefore anyone can reach us by talking to\\n\\u003ccode\\u003e2.2.2.2:4242\\u003c/code\\u003e.\\u003c/p\\u003e\\n\\u003cp\\u003eAs it turns out, that’s not always true. Some NAT devices behave\\nexactly in line with our assumptions. Their stateful firewall\\ncomponent still wants to see packets flowing in the right order, but\\nwe can reliably figure out the correct \\u003ccode\\u003eip:port\\u003c/code\\u003e to give to our peer\\nand do our simultaneous transmission trick to get through. Those NATs\\nare great, and our combination of STUN and the simultaneous packet\\nsending will work fine with those.\\u003c/p\\u003e\\n\\u003cp\\u003e(in theory, there are also NAT devices that are super relaxed, and\\ndon’t ship with stateful firewall stuff at all. In those, you don’t\\neven need simultaneous transmission, the STUN request gives you an\\ninternet \\u003ccode\\u003eip:port\\u003c/code\\u003e that anyone can connect to with no further\\nceremony. If such devices do still exist, they’re increasingly rare.)\\u003c/p\\u003e\\n\\u003cp\\u003eOther NAT devices are more difficult, and create a completely\\ndifferent NAT mapping for every different destination that you talk\\nto. On such a device, if we use the same socket to send to\\n\\u003ccode\\u003e5.5.5.5:1234\\u003c/code\\u003e and \\u003ccode\\u003e7.7.7.7:2345\\u003c/code\\u003e, we’ll end up with two different\\nports on 2.2.2.2, one for each destination. If you use the wrong port\\nto talk back, you don’t get through.\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/c9edd473a0702412836a0f0efa1024b2df60a22e-2000x1076.png\\" alt= \\"Example of when NAT devices are more difficult, and create a completely different NAT mapping for every different destination.\\" \\u003e\\n \\n\\u003c/figure\\u003e\\n\\n\\u003ch4 id=\\"naming-our-nats\\"\\u003eNaming our NATs\\u003c/h4\\u003e\\n\\u003cp\\u003eNow that we’ve discovered that not all NAT devices behave in the same\\nway, we should talk terminology. If you’ve done anything related to\\nNAT traversal before, you might have heard of “Full Cone”, “Restricted\\nCone”, “Port-Restricted Cone” and “Symmetric” NATs. These are terms\\nthat come from early research into NAT traversal.\\u003c/p\\u003e\\n\\u003cp\\u003eThat terminology is honestly quite confusing. I always look up what a\\nRestricted Cone NAT is supposed to be. Empirically, I’m not alone in\\nthis, because most of the internet calls “easy” NATs Full Cone, when\\nthese days they’re much more likely to be Port-Restricted Cone.\\u003c/p\\u003e\\n\\u003cp\\u003eMore recent research and RFCs have come up with a much better\\ntaxonomy. First of all, they recognize that there are many more\\nvarying dimensions of behavior than the single “cone” dimension of\\nearlier research, so focusing on the cone-ness of your NAT isn’t\\nnecessarily helpful. Second, they came up with words that more plainly\\nconvey what the NAT is doing.\\u003c/p\\u003e\\n\\u003cp\\u003eThe “easy” and “hard” NATs above differ in a single dimension: whether\\nor not their NAT mappings depend on what the destination is. \\u003ca href=\\"https://tools.ietf.org/html/rfc4787\\" target=\\"\_blank\\" rel=\\"noreferer noopener\\"\\u003eRFC\\n4787\\u003c/a\\u003e calls the easy variant “Endpoint-Independent Mapping”\\n(EIM for short), and the hard variant “Endpoint-Dependent Mapping”\\n(EDM for short). There’s a subcategory of EDM that specifies whether\\nthe mapping varies only on the destination IP, or on both the\\ndestination IP and port. For NAT traversal, the distinction doesn’t\\nmatter. Both kinds of EDM NATs are equally bad news for us.\\u003c/p\\u003e\\n\\u003cp\\u003eIn the grand tradition of naming things being hard,\\nendpoint-independent NATs still depend on an endpoint: each \\u003cem\\u003esource\\u003c/em\\u003e\\n\\u003ccode\\u003eip:port\\u003c/code\\u003e gets a different mapping, because otherwise your packets\\nwould get mixed up with someone else’s packets, and that would be\\nchaos. Strictly speaking, we should say “Destination Endpoint\\nIndependent Mapping” (DEIM?), but that’s a mouthful, and since “Source\\nEndpoint Independent Mapping” would be another way to say “broken”, we\\ndon’t specify. Endpoint always means “Destination Endpoint.”\\u003c/p\\u003e\\n\\u003cp\\u003eYou might be wondering how 2 kinds of endpoint dependence maps into 4\\nkinds of cone-ness. The answer is that cone-ness encompasses two\\northogonal dimensions of NAT behavior. One is NAT mapping behavior,\\nwhich we looked at above, and the other is stateful firewall\\nbehavior. Like NAT mapping behavior, the firewalls can be\\nEndpoint-Independent or a couple of variants of Endpoint-Dependent. If\\nyou throw all of these into a matrix, you can reconstruct the\\ncone-ness of a NAT from its more fundamental properties:\\u003c/p\\u003e\\n\\u003ch4 class=\\"text-center\\"\\u003eNAT Cone Types\\u003c/h4\\u003e\\n\\u003ctable\\u003e\\n \\u003cthead\\u003e\\n \\u003ctr\\u003e\\n \\u003cth\\u003e\\u003c/th\\u003e\\n \\u003cth\\u003e\\u003cstrong\\u003eEndpoint-Independent NAT\\u0026nbsp;mapping\\u003c/strong\\u003e\\u003c/th\\u003e\\n \\u003cth\\u003e\\u003cstrong\\u003eEndpoint-Dependent NAT\\u0026nbsp;mapping (all types)\\u003c/strong\\u003e\\u003c/th\\u003e\\n \\u003c/tr\\u003e\\u003c/thead\\u003e\\n \\u003ctbody\\u003e\\n \\u003ctr\\u003e\\n \\u003ctd\\u003e\\u003cstrong\\u003eEndpoint-Independent firewall\\u003c/strong\\u003e\\u003c/td\\u003e\\n \\u003ctd\\u003eFull Cone NAT\\u003c/td\\u003e\\n \\u003ctd\\u003eN/A\*\\u003c/td\\u003e\\n \\u003c/tr\\u003e\\n \\u003ctr\\u003e\\n \\u003ctd\\u003e\\u003cstrong\\u003eEndpoint-Dependent firewall (dest. IP only)\\u003c/strong\\u003e\\u003c/td\\u003e\\n \\u003ctd\\u003eRestricted Cone NAT\\u003c/td\\u003e\\n \\u003ctd\\u003eN/A\*\\u003c/td\\u003e\\n \\u003c/tr\\u003e\\n \\u003ctr\\u003e\\n \\u003ctd\\u003e\\u003cstrong\\u003eEndpoint-Dependent firewall (dest. IP+port)\\u003c/strong\\u003e\\u003c/td\\u003e\\n \\u003ctd\\u003ePort-Restricted Cone NAT\\u003c/td\\u003e\\n \\u003ctd\\u003eSymmetric NAT\\u003c/td\\u003e\\n \\u003c/tr\\u003e\\n \\u003c/tbody\\u003e\\n\\u003c/table\\u003e\\n\\u003cfigcaption\\u003e\\n \\u003cp\\u003e\* can theoretically exist, but don't show up in the wild\\u003c/p\\u003e\\n\\u003c/figcaption\\u003e\\n\\u003cp\\u003eOnce broken down like this, we can see that cone-ness isn’t terribly\\nuseful to us. The major distinction we care about is Symmetric versus\\nanything else — in other words, we care about whether a NAT device is\\nEIM or EDM.\\u003c/p\\u003e\\n\\u003cp\\u003eWhile it’s neat to know exactly how your firewall behaves, we don’t\\ncare from the point of view of writing NAT traversal code. Our\\nsimultaneous transmission trick will get through all three variants of\\nfirewalls. In the wild we’re overwhelmingly dealing only with\\nIP-and-port endpoint-dependent firewalls. So, for practical code, we\\ncan simplify the table down to:\\u003c/p\\u003e\\n\\u003ctable\\u003e\\n \\u003cthead\\u003e\\n \\u003ctr\\u003e\\n \\u003cth\\u003e\\u003c/th\\u003e\\n \\u003cth\\u003eEndpoint-Independent NAT\\u0026nbsp;mapping\\u003c/th\\u003e\\n \\u003cth\\u003eEndpoint-Dependent NAT\\u0026nbsp;mapping (dest. IP only)\\u003c/th\\u003e\\n \\u003c/tr\\u003e\\u003c/thead\\u003e\\n \\u003ctbody\\u003e\\n \\u003ctr\\u003e\\n \\u003ctd\\u003e\\u003cstrong\\u003eFirewall is yes\\u003c/strong\\u003e\\u003c/td\\u003e\\n \\u003ctd\\u003eEasy NAT\\u003c/td\\u003e\\n \\u003ctd\\u003eHard NAT\\u003c/td\\u003e\\n \\n \\u003c/tr\\u003e\\u003c/tbody\\u003e\\n\\u003c/table\\u003e\\n\\u003cp\\u003eIf you’d like to read more about the newer taxonomies of NATs, you can\\nget the full details in RFCs \\u003ca href=\\"https://tools.ietf.org/html/rfc4787\\" target=\\"\_blank\\" rel=\\"noreferer noopener\\"\\u003e4787\\u003c/a\\u003e (NAT Behavioral\\nRequirements for UDP), \\u003ca href=\\"https://tools.ietf.org/html/rfc5382\\" target=\\"\_blank\\" rel=\\"noreferer noopener\\"\\u003e5382\\u003c/a\\u003e (for TCP) and \\u003ca href=\\"https://tools.ietf.org/html/rfc5508\\" target=\\"\_blank\\" rel=\\"noreferer noopener\\"\\u003e5508\\u003c/a\\u003e\\n(for ICMP). And if you’re implementing a NAT device, these RFCs are\\nalso your guide to what behaviors you \\u003cem\\u003eshould\\u003c/em\\u003e implement, to make them\\nwell-behaved devices that play well with others and don’t generate\\ncomplaints about Halo multiplayer not working.\\u003c/p\\u003e\\n\\u003cp\\u003eBack to our NAT traversal. We were doing well with STUN and firewall\\ntraversal, but these hard NATs are a big problem. It only takes one of\\nthem in the whole path to break our current traversal plans.\\u003c/p\\u003e\\n\\u003cp\\u003eBut wait, this post is titled “how NAT traversal works”, not “how NAT\\ntraversal doesn’t work.” So presumably, I have a trick up my sleeve to\\nget out of this, right?\\u003c/p\\u003e\\n\\u003ch4 id=\\"have-you-considered-giving-up\\"\\u003eHave you considered giving up?\\u003c/h4\\u003e\\n\\u003cp\\u003eThis is a good time to have the awkward part of our chat: what happens\\nwhen we empty our entire bag of tricks, and we \\u003cem\\u003estill\\u003c/em\\u003e can’t get\\nthrough? A lot of NAT traversal code out there gives up and declares\\nconnectivity impossible. That’s obviously not acceptable for us;\\nTailscale is nothing without the connectivity.\\u003c/p\\u003e\\n\\u003cp\\u003eWe could use a relay that both sides can talk to unimpeded, and have\\nit shuffle packets back and forth. But wait, isn’t that terrible?\\u003c/p\\u003e\\n\\u003cp\\u003eSort of. It’s certainly not as good as a direct connection, but if the\\nrelay is “near enough” to the network path your direct connection\\nwould have taken, and has enough bandwidth, the impact on your\\nconnection quality isn’t huge. There will be a bit more latency, maybe\\nless bandwidth. That’s still much better than no connection at all,\\nwhich is where we were heading.\\u003c/p\\u003e\\n\\u003cp\\u003eAnd keep in mind that we only resort to this in cases where direct\\nconnections fail. We can still establish direct connections through a\\n\\u003cem\\u003elot\\u003c/em\\u003e of different networks. Having relays to handle the long tail\\nisn’t that bad.\\u003c/p\\u003e\\n\\u003cp\\u003eAdditionally, some networks can break our connectivity much more\\ndirectly than by having a difficult NAT. For example, we’ve observed\\nthat the UC Berkeley guest Wi-Fi blocks all outbound UDP except for DNS\\ntraffic. No amount of clever NAT tricks is going to get around the\\nfirewall eating your packets. So, we need some kind of reliable\\nfallback no matter what.\\u003c/p\\u003e\\n\\u003cp\\u003eYou could implement relays in a variety of ways. The classic way is a\\nprotocol called TURN (Traversal Using Relays around NAT). We’ll skip\\nthe protocol details, but the idea is that you authenticate yourself\\nto a TURN server on the internet, and it tells you “okay, I’ve\\nallocated \\u003ccode\\u003eip:port\\u003c/code\\u003e, and will relay packets for you.” You tell your\\npeer the TURN \\u003ccode\\u003eip:port\\u003c/code\\u003e, and we’re back to a completely trivial\\nclient/server communication scenario.\\u003c/p\\u003e\\n\\u003cp\\u003eFor Tailscale, we didn’t use TURN for our relays. It’s not a\\nparticularly pleasant protocol to work with, and unlike STUN there’s\\nno real interoperability benefit since there are no open TURN servers\\non the internet.\\u003c/p\\u003e\\n\\u003cp\\u003eInstead, we created \\u003ca href=\\"/blog/how-tailscale-works#encrypted-tcp-relays-derp\\"\\u003eDERP (Detoured Encrypted Routing\\nProtocol)\\u003c/a\\u003e, which is a general purpose packet relaying\\nprotocol. It runs over HTTP, which is handy on networks with strict\\noutbound rules, and relays encrypted payloads based on the\\ndestination’s public key.\\u003c/p\\u003e\\n\\u003cp\\u003eAs we briefly touched on earlier, we use this communication path both\\nas a data relay when NAT traversal fails (in the same role as TURN in\\nother systems) and as the side channel to help with NAT\\ntraversal. DERP is both our fallback of last resort to get\\nconnectivity, and our helper to upgrade to a peer-to-peer connection,\\nwhen that’s possible.\\u003c/p\\u003e\\n\\u003cp\\u003eNow that we have a relay, in addition to the traversal tricks we’ve\\ndiscussed so far, we’re in pretty good shape. We can’t get through\\n\\u003cem\\u003eeverything\\u003c/em\\u003e but we can get through quite a lot, and we have a backup\\nfor when we fail. If you stopped reading now and implemented just the\\nabove, I’d estimate you could get a direct connection over 90% of the\\ntime, and your relays guarantee \\u003cem\\u003esome\\u003c/em\\u003e connectivity all the time.\\u003c/p\\u003e\\n\\u003ch3 id=\\"nat-notes-for-nerds\\"\\u003eNAT notes for nerds\\u003c/h3\\u003e\\n\\u003cp\\u003eBut… If you’re not satisfied with “good enough”, there’s still a lot\\nmore we can do! What follows is a somewhat miscellaneous set of\\ntricks, which can help us out in specific situations. None of them\\nwill solve NAT traversal by itself, but by combining them judiciously,\\nwe can get incrementally closer to a 100% success rate.\\u003c/p\\u003e\\n\\u003ch4 id=\\"the-benefits-of-birthdays\\"\\u003eThe benefits of birthdays\\u003c/h4\\u003e\\n\\u003cp\\u003eLet’s revisit our problem with hard NATs. The key issue is that the\\nside with the easy NAT doesn’t know what \\u003ccode\\u003eip:port\\u003c/code\\u003e to send to on the\\nhard side. But \\u003cem\\u003emust\\u003c/em\\u003e send to the right \\u003ccode\\u003eip:port\\u003c/code\\u003e in order to open up\\nits firewall to return traffic. What can we do about that?\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/647364b5f593aafded475c9018f5a299f9893104-2000x760.png\\" alt= \\"Illustration of key issue when the side with the easy NAT doesn’t know what ip:port to send to on the hard side.\\" \\u003e\\n \\n\\u003c/figure\\u003e\\n\\n\\u003cp\\u003eWell, we know \\u003cem\\u003esome\\u003c/em\\u003e \\u003ccode\\u003eip:port\\u003c/code\\u003e for the hard side, because we ran\\nSTUN. Let’s assume that the IP address we got is correct. That’s not\\n\\u003cem\\u003enecessarily\\u003c/em\\u003e true, but let’s run with the assumption for now. As it\\nturns out, it’s mostly safe to assume this. (If you’re curious why,\\nsee REQ-2 in \\u003ca href=\\"https://tools.ietf.org/html/rfc4787\\" target=\\"\_blank\\" rel=\\"noreferer noopener\\"\\u003eRFC 4787\\u003c/a\\u003e.)\\u003c/p\\u003e\\n\\u003cp\\u003eIf the IP address is correct, our only unknown is the port. There’s\\n65,535 possibilities… Could we try all of them? At 100 packets/sec,\\nthat’s a worst case of 10 minutes to find the right one. It’s better\\nthan nothing, but not great. And it \\u003cem\\u003ereally\\u003c/em\\u003e looks like a port scan\\n(because in fairness, it is), which may anger network intrusion\\ndetection software.\\u003c/p\\u003e\\n\\u003cp\\u003eWe can do much better than that, with the help of the \\u003ca href=\\"https://en.wikipedia.org/wiki/Birthday\_problem\\" target=\\"\_blank\\" rel=\\"noreferer noopener\\"\\u003ebirthday\\nparadox\\u003c/a\\u003e. Rather than open 1 port on the hard side and have the\\neasy side try 65,535 possibilities, let’s open, say, 256 ports on the\\nhard side (by having 256 sockets sending to the easy side’s\\n\\u003ccode\\u003eip:port\\u003c/code\\u003e), and have the easy side probe target ports at random.\\u003c/p\\u003e\\n\\u003cp\\u003eI’ll spare you the detailed math, but you can check out the dinky\\n\\u003ca href=\\"https://github.com/danderson/nat-birthday-paradox\\" target=\\"\_blank\\" rel=\\"noreferer noopener\\"\\u003epython calculator\\u003c/a\\u003e I made while working it out. The\\ncalculation is a very slight variant on the “classic” birthday\\nparadox, because it’s looking at collisions between two sets\\ncontaining distinct elements, rather than collisions within a single\\nset. Fortunately, the difference works out slightly in our favor!\\nHere’s the chances of a collision of open ports (i.e. successful\\ncommunication), as the number of random probes from the easy side\\nincreases, and assuming 256 ports on the hard side:\\u003c/p\\u003e\\n\\u003ctable\\u003e\\n\\u003cthead\\u003e\\n\\u003ctr\\u003e\\n\\u003cth\\u003eNumber of random probes\\u003c/th\\u003e\\n\\u003cth\\u003eChance of success\\u003c/th\\u003e\\n\\u003c/tr\\u003e\\n\\u003c/thead\\u003e\\n\\u003ctbody\\u003e\\n\\u003ctr\\u003e\\n\\u003ctd\\u003e174\\u003c/td\\u003e\\n\\u003ctd\\u003e50%\\u003c/td\\u003e\\n\\u003c/tr\\u003e\\n\\u003ctr\\u003e\\n\\u003ctd\\u003e256\\u003c/td\\u003e\\n\\u003ctd\\u003e64%\\u003c/td\\u003e\\n\\u003c/tr\\u003e\\n\\u003ctr\\u003e\\n\\u003ctd\\u003e1024\\u003c/td\\u003e\\n\\u003ctd\\u003e98%\\u003c/td\\u003e\\n\\u003c/tr\\u003e\\n\\u003ctr\\u003e\\n\\u003ctd\\u003e2048\\u003c/td\\u003e\\n\\u003ctd\\u003e99.9%\\u003c/td\\u003e\\n\\u003c/tr\\u003e\\n\\u003c/tbody\\u003e\\n\\u003c/table\\u003e\\n\\u003cp\\u003eIf we stick with a fairly modest probing rate of 100 ports/sec, half\\nthe time we’ll get through in under 2 seconds. And even if we get\\nunlucky, 20 seconds in we’re virtually guaranteed to have found a way\\nin, after probing less than 4% of the total search space.\\u003c/p\\u003e\\n\\u003cp\\u003eThat’s great! With this additional trick, one hard NAT in the path is\\nan annoying speedbump, but we can manage. What about two?\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/82546090404cf5ab862b7f1ed541ee13a34d2d5a-2000x760.png\\" alt= \\"Diagram shows random destination ports probed through a hard NAT resulting in a random source port.\\"\\u003e\\n \\n\\u003c/figure\\u003e\\n\\n\\u003cp\\u003eWe can try to apply the same trick, but now the search is much harder:\\neach random destination port we probe through a hard NAT also results\\nin a random \\u003cem\\u003esource\\u003c/em\\u003e port. That means we’re now looking for a\\ncollision on a \\u003ccode\\u003e{source port, destination port}\\u003c/code\\u003e pair, rather than\\njust the destination port.\\u003c/p\\u003e\\n\\u003cp\\u003eAgain I’ll spare you the calculations, but after 20 seconds in the\\nsame regime as the previous setup (256 probes from one side, 2048 from\\nthe other), our chance of success is… 0.01%.\\u003c/p\\u003e\\n\\u003cp\\u003eThis shouldn’t be surprising if you’ve studied the birthday paradox\\nbefore. The birthday paradox lets us convert \\u003ccode\\u003eN\\u003c/code\\u003e “effort” into\\nsomething on the order of \\u003ccode\\u003esqrt(N)\\u003c/code\\u003e. But we squared the size of the\\nsearch space, so even the reduced amount of effort is still a lot more\\neffort. To hit a 99.9% chance of success, we need each side to send\\n170,000 probes. At 100 packets/sec, that’s 28 minutes of trying before\\nwe can communicate. 50% of the time we’ll succeed after “only” 54,000\\npackets, but that’s still 9 minutes of waiting around with no\\nconnection. Still, that’s better than the 1.2 \\u003cem\\u003eyears\\u003c/em\\u003e it would take\\nwithout the birthday paradox.\\u003c/p\\u003e\\n\\u003cp\\u003eIn some applications, 28 minutes might still be worth it. Spend half\\nan hour brute-forcing your way through, then you can keep pinging to\\nkeep the open path alive indefinitely — or at least until one of the\\nNATs reboots and dumps all its state, then you’re back to brute\\nforcing. But it’s not looking good for any kind of interactive\\nconnectivity.\\u003c/p\\u003e\\n\\u003cp\\u003eWorse, if you look at common office routers, you’ll find that they\\nhave a surprisingly low limit on active sessions. For example, a\\nJuniper SRX 300 maxes out at 64,000 active sessions. We’d consume its\\nentire session table with our one attempt to get through! And that’s\\nassuming the router behaves gracefully when overloaded. \\u003cem\\u003eAnd\\u003c/em\\u003e this is\\nall to get a single connection! What if we have 20 machines doing this\\nbehind the same router? Disaster.\\u003c/p\\u003e\\n\\u003cp\\u003eStill, with this trick we can make it through a slightly harder\\nnetwork topology than before. That’s a big deal, because home routers\\ntend to be easy NATs, and hard NATs tend to be office routers or cloud\\nNAT gateways. That means this trick buys us improved connectivity for\\nthe home-to-office and home-to-cloud scenarios, as well as a few\\noffice-to-cloud and cloud-to-cloud scenarios.\\u003c/p\\u003e\\n\\u003ch4 id=\\"partially-manipulating-port-maps\\"\\u003ePartially manipulating port maps\\u003c/h4\\u003e\\n\\u003cp\\u003eOur hard NATs would be so much easier if we could ask the NATs to stop\\nbeing such jerks, and let more stuff through. Turns out, there’s a\\nprotocol for that! Three of them, to be precise. Let’s talk about\\nport mapping protocols.\\u003c/p\\u003e\\n\\u003cp\\u003eThe oldest is the \\u003ca href=\\"https://openconnectivity.org/developer/specifications/upnp-resources/upnp/internet-gateway-device-igd-v-2-0/\\" target=\\"\_blank\\" rel=\\"noreferer noopener\\"\\u003eUPnP IGD\\u003c/a\\u003e (Universal Plug’n’Play Internet\\nGateway Device) protocol. It was born in the late 1990’s, and as such\\nuses a lot of very 90’s technology (XML, SOAP, multicast HTTP over UDP\\n— yes, really) and is quite hard to implement correctly and securely —\\nbut a lot of routers shipped with UPnP, and a lot still do. If we\\nstrip away all the fluff, we find a very simple request-response that\\nall three of our port mapping protocols implement: “Hi, please forward\\na WAN port to \\u003ccode\\u003elan-ip:port\\u003c/code\\u003e,” and “okay, I’ve allocated \\u003ccode\\u003ewan-ip:port\\u003c/code\\u003e\\nfor you.”\\u003c/p\\u003e\\n\\u003cp\\u003eSpeaking of stripping away the fluff: some years after UPnP IGD came\\nout, Apple launched a competing protocol called \\u003ca href=\\"https://tools.ietf.org/html/rfc6886\\" target=\\"\_blank\\" rel=\\"noreferer noopener\\"\\u003eNAT-PMP\\u003c/a\\u003e\\n(NAT Port Mapping Protocol). Unlike UPnP, it \\u003cem\\u003eonly\\u003c/em\\u003e does port\\nforwarding, and is extremely simple to implement, both on clients and\\non NAT devices. A little bit after that, NAT-PMP v2 was reborn as\\n\\u003ca href=\\"https://tools.ietf.org/html/rfc6887\\" target=\\"\_blank\\" rel=\\"noreferer noopener\\"\\u003ePCP\\u003c/a\\u003e (Port Control Protocol).\\u003c/p\\u003e\\n\\u003cp\\u003eSo, to help our connectivity further, we can look for UPnP IGD,\\nNAT-PMP and PCP on our local default gateway. If one of the protocols\\nelicits a response, we request a public port mapping. You can think of\\nthis as a sort of supercharged STUN: in addition to discovering our\\npublic \\u003ccode\\u003eip:port\\u003c/code\\u003e, we can instruct the NAT to be friendlier to our\\npeers, by not enforcing firewall rules for that port. Any packet from\\nanywhere that lands on our mapped port will make it back to us.\\u003c/p\\u003e\\n\\u003cp\\u003eYou can’t rely on these protocols being present. They might not be\\nimplemented on your devices. They might be disabled by default and\\nnobody knew to turn them on. They might have been disabled by policy.\\u003c/p\\u003e\\n\\u003cp\\u003eDisabling by policy is fairly common because UPnP suffered from a\\nnumber of high-profile vulnerabilities (since fixed, so newer devices\\ncan safely offer UPnP, if implemented properly). Unfortunately, many\\ndevices come with a single “UPnP” checkbox that actually toggles UPnP,\\nNAT-PMP and PCP all at once, so folks concerned about UPnP’s security\\nend up disabling the perfectly safe alternatives as well.\\u003c/p\\u003e\\n\\u003cp\\u003eStill, when it’s available, it effectively makes one NAT vanish from\\nthe data path, which usually makes connectivity trivial… But let’s\\nlook at the unusual cases.\\u003c/p\\u003e\\n\\u003ch4 id=\\"negotiating-numerous-nats\\"\\u003eNegotiating numerous NATs\\u003c/h4\\u003e\\n\\u003cp\\u003eSo far, the topologies we’ve looked at have each client behind one NAT\\ndevice, with the two NATs facing each other. What happens if we build\\na “double NAT”, by chaining two NATs in front of one of our machines?\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/b35c4c4e9151c88c07ef9b77daf2a1bcc97dc36b-2000x760.png\\" alt=\\"What happens if we build a “double NAT”, by chaining two NATs in front of one of our machines?\\"\\u003e\\n \\n \\u003cfigcaption\\u003e\\n \\u003cp\\u003eWhat happens if we build a “double NAT”, by chaining two NATs in front of one of our machines?\\u003c/p\\u003e\\n \\u003c/figcaption\\u003e\\n\\u003c/figure\\u003e\\n\\n\\u003cp\\u003eIn this example, not much of interest happens. Packets from client A\\ngo through two different layers of NAT on their way to the\\ninternet. But the outcome is the same as it was with multiple layers\\nof stateful firewalls: the extra layer is invisible to everyone, and\\nour other techniques will work fine regardless of how many layers\\nthere are. All that matters is the behavior of the “last” layer before\\nthe internet, because that’s the one that our peer has to find a way\\nthrough.\\u003c/p\\u003e\\n\\u003cp\\u003eThe big thing that breaks is our port mapping protocols. They act upon\\nthe layer of NAT closest to the client, whereas the one we need to\\ninfluence is the one furthest away. You can still use the port mapping\\nprotocols, but you’ll get an \\u003ccode\\u003eip:port\\u003c/code\\u003e in the “middle” network, which\\nyour remote peer cannot reach. Unfortunately, none of the protocols\\ngive you enough information to find the “next NAT up” to repeat the\\nprocess there, although you could try your luck with a traceroute and\\nsome blind requests to the next few hops.\\u003c/p\\u003e\\n\\u003cp\\u003eBreaking port mapping protocols is the reason why the internet is so\\nfull of warnings about the evils of double-NAT, and how you should\\nbend over backwards to avoid them. But in fact, double-NAT is entirely\\ninvisible to most internet-using applications, because most\\napplications don’t try to do this kind of explicit NAT traversal.\\u003c/p\\u003e\\n\\u003cp\\u003eI’m definitely not saying that you \\u003cem\\u003eshould\\u003c/em\\u003e set up a double-NAT in\\nyour network. Breaking the port mapping protocols will degrade\\nmultiplayer on many video games, and will likely strip IPv6 from your\\nnetwork, which robs you of some very good options for NAT-free\\nconnectivity. But, if circumstances beyond your control force you into\\na double-NAT, and you can live with the downsides, most things will\\nstill work fine.\\u003c/p\\u003e\\n\\u003cp\\u003eWhich is a good thing, because you know what circumstances beyond your\\ncontrol force you to double-NAT? Let’s talk carrier-grade NAT.\\u003c/p\\u003e\\n\\u003ch4 id=\\"concerning-cgnats\\"\\u003eConcerning CGNATs\\u003c/h4\\u003e\\n\\u003cp\\u003eEven with NATs to stretch the supply of IPv4 addresses, we’re \\u003cem\\u003estill\\u003c/em\\u003e\\nrunning out, and ISPs can no longer afford to give one entire public\\nIP address to every home on their network. To work around this, ISPs\\napply SNAT recursively: your home router SNATs your devices to an\\n“intermediate” IP address, and further out in the ISP’s network a\\nsecond layer of NAT devices map those intermediate IPs onto a smaller\\nnumber of public IPs. This is “carrier-grade NAT”, or CGNAT for short.\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/96c154b056a1c0a992e131423c81fcfb1e6df368-2100x1080.png\\" alt= \\"Diagram of “carrier-grade NAT” (CGNAT).\\"\\u003e\\n \\n\\u003c/figure\\u003e\\n\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/4fb6b92dd545e2e365cd5739e5ef4dbc9b554875-2100x1080.png\\" alt=\\"How do we connect two peers who are behind the same CGNAT, but different home NATs within?\\"\\u003e\\n \\n \\u003cfigcaption\\u003e\\n \\u003cp\\u003eHow do we connect two peers who are behind the same CGNAT, but different home NATs within?\\u003c/p\\u003e\\n \\u003c/figcaption\\u003e\\n\\u003c/figure\\u003e\\n\\n\\u003cp\\u003eCarrier-grade NAT is an important development for NAT traversal. Prior\\nto CGNAT, enterprising users could work around NAT traversal\\ndifficulties by manually configuring port forwarding on their home\\nrouters. But you can’t reconfigure the ISP’s CGNAT! Now even power\\nusers have to wrestle with the problems NATs pose.\\u003c/p\\u003e\\n\\u003cp\\u003eThe good news: this is a run of the mill double-NAT, and so as we\\ncovered above it’s mostly okay. Some stuff won’t work as well as it\\ncould, but things work well enough that ISPs can charge money for\\nit. Aside from the port mapping protocols, everything from our current\\nbag of tricks works fine in a CGNAT world.\\u003c/p\\u003e\\n\\u003cp\\u003eWe do have to overcome a new challenge, however: how do we connect two\\npeers who are behind the same CGNAT, but different home NATs within?\\nThat’s how we set up peers A and B in the diagram above.\\u003c/p\\u003e\\n\\u003cp\\u003eThe problem here is that STUN doesn’t work the way we’d like. We’d\\nlike to find out our \\u003ccode\\u003eip:port\\u003c/code\\u003e on the “middle network”, because it’s\\neffectively playing the role of a miniature internet to our two\\npeers. But STUN tells us what our \\u003ccode\\u003eip:port\\u003c/code\\u003e is from the STUN server’s\\npoint of view, and the STUN server is out on the internet, beyond the\\nCGNAT.\\u003c/p\\u003e\\n\\u003cp\\u003eIf you’re thinking that port mapping protocols can help us here,\\nyou’re right! If either peer’s home NAT supports one of the port\\nmapping protocols, we’re happy, because we have an \\u003ccode\\u003eip:port\\u003c/code\\u003e that\\nbehaves like an un-NATed server, and connecting is\\ntrivial. Ironically, the fact that double NAT “breaks” the port\\nmapping protocols helps us! Of course, we still can’t count on these\\nprotocols helping us out, doubly so because CGNAT ISPs tend to turn\\nthem off in the equipment they put in homes in order to avoid software\\ngetting confused by the “wrong” results they would get.\\u003c/p\\u003e\\n\\u003cp\\u003eBut what if we don’t get lucky, and can’t map ports on our NATs? Let’s\\ngo back to our STUN-based technique and see what happens. Both peers\\nare behind the same CGNAT, so let’s say that STUN tells us that peer A\\nis \\u003ccode\\u003e2.2.2.2:1234\\u003c/code\\u003e, and peer B is \\u003ccode\\u003e2.2.2.2:5678\\u003c/code\\u003e.\\u003c/p\\u003e\\n\\u003cp\\u003eThe question is: what happens when peer A sends a packet to\\n\\u003ccode\\u003e2.2.2.2:5678\\u003c/code\\u003e? We might hope that the following takes place in the\\nCGNAT box:\\u003c/p\\u003e\\n\\u003cul\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eApply peer A’s NAT mapping, rewrite the packet to be from \\u003ccode\\u003e2.2.2.2:1234\\u003c/code\\u003e and\\nto \\u003ccode\\u003e2.2.2.2:5678\\u003c/code\\u003e.\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eNotice that \\u003ccode\\u003e2.2.2.2:5678\\u003c/code\\u003e matches peer B’s \\u003cem\\u003eincoming\\u003c/em\\u003e NAT mapping, rewrite\\nthe packet to be from \\u003ccode\\u003e2.2.2.2:1234\\u003c/code\\u003e and to peer B’s private IP.\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eSend the packet on to peer B, on the “internal” interface rather than off\\ntowards the internet.\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003c/ul\\u003e\\n\\u003cp\\u003eThis behavior of NATs is called hairpinning, and with all this\\ndramatic buildup you won’t be surprised to learn that hairpinning\\nworks on some NATs and not others.\\u003c/p\\u003e\\n\\u003cp\\u003eIn fact, a great many otherwise well-behaved NAT devices don’t support\\nhairpinning, because they make assumptions like “a packet from my\\ninternal network to a non-internal IP address will always flow\\noutwards to the internet”, and so end up dropping packets as they try\\nto turn around within the router. These assumptions might even be\\nbaked into routing silicon, where it’s impossible to fix without new\\nhardware.\\u003c/p\\u003e\\n\\u003cp\\u003eHairpinning, or lack thereof, is a trait of all NATs, not just\\nCGNATs. In most cases, it doesn’t matter, because you’d expect two LAN\\ndevices to talk directly to each other rather than hairpin through\\ntheir default gateway. And it’s a pity that it usually doesn’t matter,\\nbecause that’s probably why hairpinning is commonly broken.\\u003c/p\\u003e\\n\\u003cp\\u003eBut once CGNAT is involved, hairpinning becomes vital to\\nconnectivity. Hairpinning lets you apply the same tricks that you use\\nfor internet connectivity, without worrying about whether you’re\\nbehind a CGNAT. If both hairpinning and port mapping protocols fail,\\nyou’re stuck with relaying.\\u003c/p\\u003e\\n\\u003ch4 id=\\"ideally-ipv6-nat64-notwithstanding\\"\\u003eIdeally IPv6, NAT64 notwithstanding\\u003c/h4\\u003e\\n\\u003cp\\u003eBy this point I expect some of you are shouting at your screens that\\nthe solution to all this nonsense is IPv6. All this is happening\\nbecause we’re running out of IPv4 addresses, and we keep piling on\\nNATs to work around that. A much simpler fix would be to not have an IP\\naddress shortage, and make every device in the world reachable without\\nNATs. Which is exactly what IPv6 gets us.\\u003c/p\\u003e\\n\\u003cp\\u003eAnd you’re right! Sort of. It’s true that in an IPv6-only world, all\\nof this becomes much simpler. Not trivial, mind you, because we’re\\nstill stuck with stateful firewalls. Your office workstation may have\\na globally reachable IPv6 address, but I’ll bet there’s still a\\ncorporate firewall enforcing “outbound connections only” between you\\nand the greater internet. And on-device firewalls are still there,\\nenforcing the same thing.\\u003c/p\\u003e\\n\\u003cp\\u003eSo, we still need the firewall traversal stuff from the start of the\\narticle, and a side channel so that peers can know what \\u003ccode\\u003eip:port\\u003c/code\\u003e to\\ntalk to. We’ll probably also still want fallback relays that use a\\nwell-like protocol like HTTP, to get out of networks that block\\noutbound UDP. But we can get rid of STUN, the birthday paradox trick,\\nport mapping protocols, and all the hairpinning bumf. That’s much\\nnicer!\\u003c/p\\u003e\\n\\u003cp\\u003eThe big catch is that we currently don’t have an all-IPv6 world. We\\nhave a world that’s mostly IPv4, and \\u003ca href=\\"https://www.google.com/intl/en/ipv6/statistics.html\\" target=\\"\_blank\\" rel=\\"noreferer noopener\\"\\u003eabout 33% IPv6\\u003c/a\\u003e. Those\\n34% are very unevenly distributed, so a particular set of peers could\\nbe 100% IPv6, 0% IPv6, or anywhere in between.\\u003c/p\\u003e\\n\\u003cp\\u003eWhat this means, unfortunately, is that IPv6 isn’t \\u003cem\\u003eyet\\u003c/em\\u003e the solution\\nto our problems. For now, it’s just an extra tool in our connectivity\\ntoolbox. It’ll work fantastically well with some pairs of peers, and\\nnot at all for others. If we’re aiming for “connectivity no matter\\nwhat”, we have to also do IPv4+NAT stuff.\\u003c/p\\u003e\\n\\u003cp\\u003eMeanwhile, the coexistence of IPv6 and IPv4 introduces yet another new\\nscenario we have to account for: NAT64 devices.\\u003c/p\\u003e\\n\\n \\n \\n\\n\\u003cfigure class=\\"image-wide\\"\\u003e\\n \\u003cimg src=\\"https://cdn.sanity.io/images/w77i7m8x/production/d84e6f36bd77b3cd3b0524c7fac9a5c17fb96f05-2000x900.png\\" alt=\\"Diagram shows the coexistence of IPv6 and IPv4 introducing a scenario for NAT64 devices.\\" \\u003e\\n \\n\\u003c/figure\\u003e\\n\\n\\u003cp\\u003eSo far, the NATs we’ve looked at have been NAT44: they translate IPv4\\naddresses on one side to different IPv4 addresses on the other\\nside. NAT64, as you might guess, translates between protocols. IPv6 on\\nthe internal side of the NAT becomes IPv4 on the external\\nside. Combined with DNS64 to translate IPv4 DNS answers into IPv6, you\\ncan present an IPv6-only network to the end device, while still giving\\naccess to the IPv4 internet.\\u003c/p\\u003e\\n\\u003cp\\u003e(Incidentally, you can extend this naming scheme indefinitely. There\\nhave been some experiments with NAT46; you could deploy NAT66 if you\\nenjoy chaos; and some RFCs use NAT444 for carrier-grade NAT.)\\u003c/p\\u003e\\n\\u003cp\\u003eThis works fine if you only deal in DNS names. If you connect to\\ngoogle.com, turning that into an IP address involves the DNS64\\napparatus, which lets the NAT64 get involved without you being any the\\nwiser.\\u003c/p\\u003e\\n\\u003cp\\u003eBut we care deeply about specific IPs and ports for our NAT and\\nfirewall traversal. What about us? If we’re lucky, our device supports\\nCLAT (Customer-side translator — from Customer XLAT). CLAT makes the\\nOS pretend that it has direct IPv4 connectivity, using NAT64 behind\\nthe scenes to make it work out. On CLAT devices, we don’t need to do\\nanything special.\\u003c/p\\u003e\\n\\u003cp\\u003eCLAT is very common on mobile devices, but very uncommon on desktops,\\nlaptops and servers. On those, we have to explicitly do the work CLAT\\nwould have done: detect the existence of a NAT64+DNS64 setup, and use\\nit appropriately.\\u003c/p\\u003e\\n\\u003cp\\u003eDetecting NAT64+DNS64 is easy: send a DNS request to \\u003ccode\\u003eipv4only.arpa.\\u003c/code\\u003e\\nThat name resolves to known, constant IPv4 addresses, and only IPv4\\naddresses. If you get IPv6 addresses back, you know that a DNS64 did\\nsome translation to steer you to a NAT64. That lets you figure out\\nwhat the NAT64 prefix is.\\u003c/p\\u003e\\n\\u003cp\\u003eFrom there, to talk to IPv4 addresses, send IPv6 packets to \\u003ccode\\u003e{NAT64 prefix + IPv4 address}\\u003c/code\\u003e. Similarly, if you receive traffic from\\n\\u003ccode\\u003e{NAT64 prefix + IPv4 address}\\u003c/code\\u003e, that’s IPv4 traffic. Now speak STUN\\nthrough the NAT64 to discover your public \\u003ccode\\u003eip:port\\u003c/code\\u003e on the NAT64, and\\nyou’re back to the classic NAT traversal problem — albeit with a bit\\nmore work.\\u003c/p\\u003e\\n\\u003cp\\u003eFortunately for us, this is a fairly esoteric corner case. Most\\nv6-only networks today are mobile operators, and almost all phones\\nsupport CLAT. ISPs running v6-only networks deploy CLAT on the router\\nthey give you, and again you end up none the wiser. But if you want to\\nget those last few opportunities for connectivity, you’ll have to\\nexplicitly support talking to v4-only peers from a v6-only network as\\nwell.\\u003c/p\\u003e\\n\\u003ch3 id=\\"integrating-it-all-with-ice\\"\\u003eIntegrating it all with ICE\\u003c/h3\\u003e\\n\\u003cp\\u003eWe’re in the home stretch. We’ve covered stateful firewalls, simple\\nand advanced NAT tricks, IPv4 and IPv6. So, implement all the above,\\nand we’re done!\\u003c/p\\u003e\\n\\u003cp\\u003eExcept, how do you figure out which tricks to use for a particular\\npeer? How do you figure out if this is a simple stateful firewall\\nproblem, or if it’s time to bust out the birthday paradox, or if you\\nneed to fiddle with NAT64 by hand? Or maybe the two of you are on the\\nsame Wi-Fi network, with no firewalls and no effort required.\\u003c/p\\u003e\\n\\u003cp\\u003eEarly research into NAT traversal had you precisely characterize the\\npath between you and your peer, and deploy a specific set of\\nworkarounds to defeat that exact path. But as it turned out, network\\nengineers and NAT box programmers have many inventive ideas, and that\\nstops scaling very quickly. We need something that involves a bit less\\nthinking on our part.\\u003c/p\\u003e\\n\\u003cp\\u003eEnter the Interactive Connectivity Establishment (ICE) protocol. Like\\nSTUN and TURN, ICE has its roots in the telephony world, and so the\\nRFC is full of SIP and SDP and signalling sessions and dialing and so\\nforth. However, if you push past that, it also specifies a stunningly\\nelegant algorithm for figuring out the best way to get a connection.\\u003c/p\\u003e\\n\\u003cp\\u003eReady? The algorithm is: try everything at once, and pick the best\\nthing that works. That’s it. Isn’t that amazing?\\u003c/p\\u003e\\n\\u003cp\\u003eLet’s look at this algorithm in a bit more detail. We’re going to\\ndeviate from the ICE spec here and there, so if you’re trying to\\nimplement an interoperable ICE client, you should go read \\u003ca href=\\"https://tools.ietf.org/html/rfc8445\\" target=\\"\_blank\\" rel=\\"noreferer noopener\\"\\u003eRFC\\n8445\\u003c/a\\u003e and implement that. We’ll skip all the\\ntelephony-oriented stuff to focus on the core logic, and suggest a few\\nplaces where you have more degrees of freedom than the ICE spec\\nsuggests.\\u003c/p\\u003e\\n\\u003cp\\u003eTo communicate with a peer, we start by gathering a list of candidate\\nendpoints for our local socket. A candidate is any \\u003ccode\\u003eip:port\\u003c/code\\u003e that\\nour peer might, perhaps, be able to use in order to speak to us. We\\ndon’t need to be picky at this stage, the list should include at\\nleast:\\u003c/p\\u003e\\n\\u003cul\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eIPv6 \\u003ccode\\u003eip:ports\\u003c/code\\u003e\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eIPv4 LAN \\u003ccode\\u003eip:ports\\u003c/code\\u003e\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eIPv4 WAN \\u003ccode\\u003eip:ports\\u003c/code\\u003e discovered by STUN (possibly via a NAT64 translator)\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eIPv4 WAN \\u003ccode\\u003eip:port\\u003c/code\\u003e allocated by a port mapping protocol\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eOperator-provided endpoints (e.g. for statically configured port forwards)\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003c/ul\\u003e\\n\\u003cp\\u003eThen, we swap candidate lists with our peer through the side channel,\\nand start sending probe packets at each others’ endpoints. Again, at\\nthis point you don’t discriminate: if the peer provided you with 15\\nendpoints, you send “are you there?” probes to all 15 of them.\\u003c/p\\u003e\\n\\u003cp\\u003eThese packets are pulling double duty. Their first function is to act\\nas the packets that open up the firewalls and pierce the NATs, like\\nwe’ve been doing for this entire article. But the other is to act as a\\nhealth check. We’re exchanging (hopefully authenticated) “ping” and\\n“pong” packets, to check if a particular path works end to end.\\u003c/p\\u003e\\n\\u003cp\\u003eFinally, after some time has passed, we pick the “best” (according to\\nsome heuristic) candidate path that was observed to work, and we’re\\ndone.\\u003c/p\\u003e\\n\\u003cp\\u003eThe beauty of this algorithm is that if your heuristic is right,\\nyou’ll always get an optimal answer. ICE has you score your candidates\\nahead of time (usually: LAN \\u0026gt; WAN \\u0026gt; WAN+NAT), but it doesn’t have to\\nbe that way. Starting with v0.100.0, Tailscale switched from a\\nhardcoded preference order to round-trip latency, which tends to\\nresult in the same LAN \\u0026gt; WAN \\u0026gt; WAN+NAT ordering. But unlike static\\nordering, we discover which “category” a path falls into organically,\\nrather than having to guess ahead of time.\\u003c/p\\u003e\\n\\u003cp\\u003eThe ICE spec structures the protocol as a “probe phase” followed by an\\n“okay let’s communicate” phase, but there’s no reason you \\u003cem\\u003eneed\\u003c/em\\u003e to\\nstrictly order them. In Tailscale, we upgrade connections on the fly\\nas we discover better paths, and all connections start out with DERP\\npreselected. That means you can use the connection immediately through\\nthe fallback path, while path discovery runs in parallel. Usually,\\nafter a few seconds, we’ll have found a better path, and your\\nconnection transparently upgrades to it.\\u003c/p\\u003e\\n\\u003cp\\u003eOne thing to be wary of is asymmetric paths. ICE goes to some effort\\nto ensure that both peers have picked the same network path, so that\\nthere’s definite bidirectional packet flow to keep all the NATs and\\nfirewalls open. You don’t have to go to the same effort, but you \\u003cem\\u003edo\\u003c/em\\u003e\\nhave to ensure that there’s bidirectional traffic along all paths\\nyou’re using. That can be as simple as continuing to send ping/pong\\nprobes periodically.\\u003c/p\\u003e\\n\\u003cp\\u003eTo be really robust, you also need to detect that your currently\\nselected path has failed (say, because maintenance caused your NAT’s\\nstate to get dumped on the floor), and downgrade to another path. You\\ncan do this by continuing to probe all possible paths and keep a set\\nof “warm” fallbacks ready to go, but downgrades are rare enough that\\nit’s probably more efficient to fall all the way back to your relay of\\nlast resort, then restart path discovery.\\u003c/p\\u003e\\n\\u003cp\\u003eFinally, we should mention security. Throughout this article, I’ve\\nassumed that the “upper layer” protocol you’ll be running over this\\nconnection brings its own security (QUIC has TLS certs, WireGuard has\\nits own public keys…). If that’s not the case, you absolutely need\\nto bring your own. Once you’re dynamically switching paths at runtime,\\nIP-based security becomes meaningless (not that it was worth much in\\nthe first place), and you \\u003cem\\u003emust\\u003c/em\\u003e have at least end-to-end\\nauthentication.\\u003c/p\\u003e\\n\\u003cp\\u003eIf you have security for your upper layer, strictly speaking it’s okay\\nif your ping/pong probes are spoofable. The worst that can happen is\\nthat an attacker can persuade you to relay your traffic through\\nthem. In the presence of e2e security, that’s not a \\u003cem\\u003ehuge\\u003c/em\\u003e deal\\n(although obviously it depends on your threat model). But for good\\nmeasure, you might as well authenticate and encrypt the path discovery\\npackets as well. Consult your local application security engineer for\\nhow to do that safely.\\u003c/p\\u003e\\n\\u003ch3 id=\\"concluding-our-connectivity-chat\\"\\u003eConcluding our connectivity chat\\u003c/h3\\u003e\\n\\u003cp\\u003eAt last, we’re done. If you implement all of the above, you’ll have\\nstate of the art NAT traversal software that can get direct\\nconnections established in the widest possible array of\\nsituations. And you’ll have your relay network to pick up the slack\\nwhen traversal fails, as it likely will for a long tail of cases.\\u003c/p\\u003e\\n\\u003cp\\u003eThis is all quite complicated! It’s one of those problems that’s fun\\nto explore, but quite fiddly to get right, especially if you start\\nchasing the long tail of opportunities for just that little bit more\\nconnectivity.\\u003c/p\\u003e\\n\\u003cp\\u003eThe good news is that, once you’ve done it, you have something of a\\nsuperpower: you get to explore the exciting and relatively\\nunder-explored world of peer-to-peer applications. So many interesting\\nideas for decentralized software fall at the first hurdle, when it\\nturns out that talking to each other on the internet is harder than\\nexpected. But now you know how to get past that, so go build cool\\nstuff!\\u003c/p\\u003e\\n\\u003cp\\u003e\\u003cstrong\\u003eHere’s a parting “TL;DR” recap:\\u003c/strong\\u003e For robust NAT traversal, you need\\nthe following ingredients:\\u003c/p\\u003e\\n\\u003cul\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eA UDP-based protocol to augment\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eDirect access to a socket in your program\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eA communication side channel with your peers\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eA couple of STUN servers\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eA network of fallback relays (optional, but highly recommended)\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003c/ul\\u003e\\n\\u003cp\\u003eThen, you need to:\\u003c/p\\u003e\\n\\u003cul\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eEnumerate all the \\u003ccode\\u003eip:ports\\u003c/code\\u003e for your socket on your directly\\nconnected interfaces\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eQuery STUN servers to discover WAN \\u003ccode\\u003eip:ports\\u003c/code\\u003e and the “difficulty”\\nof your NAT, if any\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eTry using the port mapping protocols to find more WAN \\u003ccode\\u003eip:ports\\u003c/code\\u003e\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eCheck for NAT64 and discover a WAN \\u003ccode\\u003eip:port\\u003c/code\\u003e through that as well,\\nif applicable\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eExchange all those \\u003ccode\\u003eip:ports\\u003c/code\\u003e with your peer through your side\\nchannel, along with some cryptographic keys to secure everything.\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eBegin communicating with your peer through fallback relays\\n(optional, for quick connection establishment)\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eProbe all of your peer’s \\u003ccode\\u003eip:ports\\u003c/code\\u003e for connectivity and if\\nnecessary/desired, also execute birthday attacks to get through\\nharder NATs\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eAs you discover connectivity paths that are better than the one\\nyou’re currently using, transparently upgrade away from the previous\\npaths.\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eIf the active path stops working, downgrade as needed to maintain\\nconnectivity.\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003cli\\u003e\\n\\u003cp\\u003eMake sure everything is encrypted and authenticated end-to-end.\\u003c/p\\u003e\\n\\u003c/li\\u003e\\n\\u003c/ul\\u003e\\n ","featuredImage":{"\_type":"image","alt":"Branded artwork in greyscale","asset":{"\_ref":"image-f94042c9459e89db55090dc0203b042bdc2c5937-600x315-svg","\_type":"reference"}},"authors":[{"\_rev":"Kw7ZywWq2biwgCkntZOsLB","\_type":"teamMember","member":{"name":"David Anderson","socialLink":"https://twitter.com/dave\_universetf","image":{"\_type":"sanityImage","alt":"David Anderson","asset":{"\_ref":"image-c1d565287da262060d5300fef5cbce3953d78617-1200x1197-jpg","\_type":"reference"}}},"\_id":"67695046-5dd3-4ac9-9565-d3071955c750","title":"David Anderson","\_updatedAt":"2024-03-21T16:18:28Z","\_createdAt":"2023-10-23T14:30:56Z"}],"excerpt":"Here we cover how we can get through NATs (Network Address Translators) and connect your devices directly to each other, no matter what’s standing between them. ","postMarkdown":null,"publishedAtRaw":"2020-08-21T00:00:00Z","readTime":null},"pageType":"post"},"\_\_N\_SSG":true},"page":"/[...slug]","query":{"slug":["blog","how-nat-traversal-works"]},"buildId":"4BPQOo1w8FX58n8P68myT","isFallback":false,"isExperimentalCompile":false,"gsp":true,"scriptLoader":[]}