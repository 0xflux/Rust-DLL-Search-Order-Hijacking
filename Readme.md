# Rust DLL Search Order Hijacking

Check this out on my [blog here](https://fluxsec.red/rust-dll-search-order-hijacking)!

[DLL Search Order Hijacking](https://attack.mitre.org/techniques/T1574/001/) is a technique 
[used by nation state APT threat actors](https://www.crowdstrike.com/en-us/blog/overwatch-exposes-aquatic-panda-in-possession-of-log-4-shell-exploit-tools/), 
[cyber criminals](https://www.microsoft.com/en-us/security/blog/2022/05/09/ransomware-as-a-service-understanding-the-cybercrime-gig-economy-and-how-to-protect-yourself/), 
and red and purple teams.

DLL Search Order Hijacking occurs where we exploit the 'search order' path of how Windows will look for and load modules. When an application tries to find a DLL, it will search in the **following order**:

1) The directory where the application is being launched.
2) "C:\Windows\System32".
3) "C:\Windows\System".
4) "C:\Windows".
5) Current working directory.
6) Directories in SYSTEM PATH.
7) Directories in the user's PATH.

So, if we are able to place our malicious DLL with the same name as what the program is looking for, in a directory higher than where it exists, we can load our malicious DLL instead of the genuine DLL being found lower down in the list.