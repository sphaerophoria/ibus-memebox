Skip to content
Logo Pentest-Tools.com1 Free Scans
FEATURES ▼
PRICING
SERVICES
RESOURCES ▼
COMPANY ▼
LOGIN
SIGN UP
Latest scan: Website Scanner: https://support.techsmith.com/
Network Vulnerability Scan with OpenVAS
Enter target to scan for free: |
https://support.techsmith.com/
Please wait for the previous scan to finish before starting a new scan.
Light Scan    ( Free )
Full Scan
About this scanner
Discovers outdated network services, missing security patches, badly configured servers and many other vulnerabilities.

The Network Vulnerability Scanner with OpenVAS (Full Scan) is our solution for assessing the network perimeter and for evaluating the external security posture of a company.
The scanner offers a highly simplified and easy-to-use interface over OpenVAS, the best open-source network security scanner.
It performs an in-depth network vulnerability scan by using more than 57,000 plugins. See the Technical Details below.

The Light version of the scanner is a free and very fast online tool which detects the CVEs that affect the network services of a target system, based on their version (ex. Apache 2.4.10). The scanner starts by detecting the open ports and services, and then continues by querying a database for known vulnerabilities which may affect the specific software versions. Start a Free Light Scan to see a sample output.



Sample report
Need to see the full results?
Unlock the full power and feature of our Network Vulnerability Scan with OpenVAS! Compare pricing plans and discover more tools and features.

Sign up
Sample Report
Here is a sample report for the Network Vulnerability Scan with OpenVAS (Full Scan):

Shows a summary of the vulnerabilities found in your network, the risk rating, and CVSS score
Includes technical details for each vulnerability discovered
Provides risk level information for each network vulnerability
Offers recommendations and insights on how to remediate these security flaws
Download a Full Sample Report

Sample report
Use Cases for the Network Vulnerability Scan with OpenVAS
Since the scanner allows you to detect a wide range of vulnerabilities in network services, operating systems and also in web servers, its use cases are very diverse

Infrastructure Penetration Testing
The Network Vulnerability Scanner gives you a complete picture of the 'low hanging fruits' so you can concentrate on more advanced tests. Having it online and preconfigured makes it very easy to use and it saves you precious time and effort.

Self-Security Assessment
If you need a thorough intrastructure test, this is the right tool to use. From weak passwords to missing security patches and misconfigured web servers, these types of vulnerabilities can be easily detected by our full network vulnerability assessment tool.

Third-Party Infrastructure Audit
If you are an IT services company, you can also show this report to your clients and prove that you have implemented the proper security measures to the infrastructure that you are managing.

Technical Details

About
What is a Network Vulnerability Scanner?
The network perimeter of a company is the 'wall' which isolates the internal network from the outside world. However, because the outside world needs to access various resources of the company (ex. the website), the network perimeter exposes some network services (ex. FTP, VPN, DNS, HTTP and others).

A Network Vulnerability Scanner is designed to map all the services exposed on the network perimeter and detect if they are affected by vulnerabilities.

Details about our scanner
The Light version of our Network Vulnerability Scanner performs a very fast security assessment with minimum interaction with the target system. It starts by first running Nmap in order to detect the open ports and services. Then, based on the results returned by Nmap, our network scanner interrogates a database with known vulnerabilities in order to see if the specific versions of the services are affected by any issues.
This detection method, while being very fast, it is prone to returning false positives because it relies only on the version reported by the services (which may be inaccurrate).

The Full version of the Network Vulnerability Scanner uses OpenVAS as scanning engine. OpenVAS is the most advanced open source vulnerability scanner, which is able to actively detect thousands of vulnerabilities in network services such as: SMTP, DNS, VPN, SSH, RDP, VNC, HTTP and many more. OpenVAS does vulnerability detection by connecting to each network service and sending crafted packets in order to make them respond in certain ways. Depending on the response, the scanner reports the service as vulnerable or not.

We have pre-configured and fine-tuned OpenVAS on our servers and have also added a very simple interface on top of its complex functionalities. The engine is running on a distributed environment and it is able to perform multiple parallel scans.

How it works
How does OpenVAS scanner work?
OpenVAS is a fork of the old Nessus scanner, performed in 2005 when Nessus became a commercial product. OpenVAS is currently developed and maintained by Greenbone Networks with support from the community.

OpenVAS implements each test in a plugin called NVT (Network Vulnerability Test) which is written in a scripting language called NASL (Nessus Attack Scripting Language). It currently has more than 57000 active plugins which can detect a huge number of vulnerabilities for numerous services and applications.

For instance, here is how one simple NVT looks like. It is called fortigate_detect.nasl and it tells if the target device is a Fortigate Firewall:
#
#  This script was written by David Maciejak 
#  This script is released under the GNU GPL v2
#

if(description)
{
 script_id(17367);
 script_name("Fortinet Fortigate console management detection");
 script_family("General");
 script_dependencies("http_version.nasl");
 script_require_ports(443);
 exit(0);
}

#
# The script code starts here
#
include("http_func.inc");

function https_get(port, request)
{
    if(get_port_state(port))
    {

         soc = open_sock_tcp(port, transport:ENCAPS_SSLv23);
         if(soc)
         {
            send(socket:soc, data:string(request,"\r\n"));
            result = http_recv(socket:soc);
            close(soc);
            return(result);
         }
    }
}

port = 443;

if(get_port_state(port))
{
  req1 = http_get(item:"/system/console?version=1.5", port:port);
  req = https_get(request:req1, port:port);
  #<title>Fortigate Console Access</title>

  if("Fortigate Console Access" >< req)
  {
    security_note(port);
  }
}
            

OpenVAS Scanning Policy
While OpenVAS has multiple predefined policies, our scanner uses the one called Full and Fast. This policy uses the majority of the NVTs and it is optimized to use the information collected by the previous plugins. For instance, if a previous plugin detects the FTP service running on port 2121, it will run all the FTP related plugins on that port. Otherwise it won't.

Open Ports Detection
We have configured OpenVAS to scan for a default list of ports containing the most common 6000 ports (TCP and UDP). However, please note that the scanner first attempts to detect if the host is alive or not before doing the port scan. If the host is not alive (ex. does not respond to ICMP requests) it will show zero open ports found.

Note: If the scanner does not find any open ports even though you know there are, we recommend you re-running the scan with the option "Check if host is alive" disabled. This will skip host discovery and just start the port scan.

How long does an OpenVAS scan take?
Since the OpenVAS scanner performs a considerable number of tests, the full scan can take from 30 minutes to several hours. It highly depends on the number of open ports found on the target host. As this number is larger, the scanning time increases because OpenVAS will have to run a higher number of NVTs.
TOOLS
Information Gathering
Web App Testing
Network Testing
Exploit Helpers
DEVELOPERS
API Reference
RESOURCES
Blog
Platform Tutorials
Platform Updates
Data Security
Support
LEGAL
Terms and Conditions
Privacy Policy
COMPANY
About
Team
Jobs
Contact
Logo Pentest-Tools.com
 
© 2021 Pentest-Tools.com   
Pentest-Tools.com is a corporate member of OWASP
Pentest-Tools.com is a Corporate Member of OWASP (The Open Web Application Security Project). We share their mission to use, strengthen, and advocate for secure coding standards into every piece of software we develop.

Pentest-Tools.com is a High Performer in G2’s Winter 2021 Grid® Report
Pentest-Tools.com recognized as a High Performer in G2’s Winter 2021 Grid® Report. Discover why security and IT pros worldwide use the platform to streamline their penetration and security testing workflow.
