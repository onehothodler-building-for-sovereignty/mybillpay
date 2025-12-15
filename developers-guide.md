# Bitcoin Bill Payment Module Development Guide (StartOS + WSL2)  

This document provides a full setup and development guide for building a Bitcoin Bill Payment module on StartOS using a Windows machine with WSL2.  

---  

## Post‑Install Checklist (WSL2 + StartOS Dev Environment)  

### 1. Update & Upgrade Linux  

sudo apt update && sudo apt upgrade -y  

---  

### 2. Install Rust Toolchain  
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  
rustc --version  

---  
### 3. Install Node.js & npm/yarn  

sudo apt install -y nodejs npm  
npm install --global yarn  

---  

### 4. Install Docker (WSL2 Integration)  

When using Docker Desktop with WSL2 
• Make sure Docker Desktop is installed on Windows.  
• In Docker Desktop settings, enable WSL2 integration for your Ubuntu distro.  
• Restart Docker Desktop and WSL2.  

sudo apt install -y docker.io  
sudo service docker start    
sudo usermod -aG docker $USER  

---  
### 5. Install StartOS SDK

git clone https://github.com/Start9Labs/start-os.git && \  
cd start-os && git submodule update --init --recursive && \  
make sdk 
---  

### 6. StartOS SDK Build Instructions for Packaging  

Install these inside your WSL2 environment:  

Check for apt updates first  
sudo apt update  

Install json processor jq  
sudo apt install -y jq   

Install the Deno runtime — a secure, modern alternative to Node.js for running JavaScript and TypeScript  
sudo snap install deno  

cd start-os  
make sdk  

B. Initialize the SDK  
Set up your developer key:  

start-sdk init  

By default, this creates a developer key at "/etc/embassy".  

If permissions are an issue, change ownership:

sudo chown <user> /etc/embassy 

Alternatively, you can specify a custom path in a config file:  

developer-key-path: /desired/path/to/key

Then run:  
start-sdk -c /path/to/config init  

C. Verify Installation  

Check the SDK version:  

start-sdk --version  

List available commands:  

start-sdk --help  

D. Package Your Service  
Ensure the following files exist in your service directory:  

* manifest.yaml  
* instructions.md  
* LICESNE.md  
* icon.png (Under 100KB)  
* Dockerfile  

Run the packaging command which assembles everything into a file    

start-sdk pack  

E. Verify the Package  

Run:  

start-sdk verify  

This checks:  
• 	All mounts are valid volumes in the manifest  
• 	Cert volumes point to real interfaces  
• 	Actions refer to real images  
• 	Images are tagged correctly  
• 	Icon size is valid  

F. Inspect the Package (Optional)  
You can inspect components inside the :  

start-sdk inspect manifest /path/to/<package-id>.s9pk  
start-sdk inspect instructions /path/to/<package-id>.s9pk  

---

### 7. Verify Environment  

Test Rust:  
cargo new testapp && cd testapp && cargo build

Test Node.js:  
node -v && npm -v  

Test Docker:  
docker run hello-world

---

### 8. Prepare Project Workspace  

Organize your project into:  
• Rust service logic  
• React/Vue dashboard  
• StartOS manifest + s9pk build file  


Stepwise Development Playbook  
1. Environment Setup  
• 	Install WSL2 (Ubuntu 22.04 recommended)  
• 	Install Rust toolchain via rustup inside WSL2  
• 	Install Docker Desktop with WSL2 integration (or Podman)  
• 	Install Node.js and npm/yarn for React or Vue frontend   development  
• 	Clone the StartOS SDK from GitHub for s9pk packaging  
2. Backend Development (Rust)  
• 	Create a new Rust project with   
• 	Implement core logic: Lightning payments, on-chain BTC   transactions, fiat bridge (ACH or swap)  
• 	Add crates: tokio, serde, reqwest  
• 	Write REST or gRPC endpoints to expose payment functions  
3. Frontend Development (React/Vue)  
• 	Initialize frontend with  or   
• 	Build UI components: bill list, payment form, transaction   history, notifications  
• 	Connect frontend to backend via REST or gRPC APIs  
• 	Bundle frontend into Docker container for deployment  
4. Service Packaging (s9pk)  
• 	Write  defining service name, version, dependencies, ports, permissions  
• 	Use StartOS SDK to build and sign the  package  
• 	Test package installation locally in WSL2 or on a StartOS VM  
5. Testing and Validation  
• 	Run integration tests: Lightning invoice payment, on-chain BTC transaction, ACH payout  
• 	Validate UI flows: bill scheduling, payment confirmation  
• 	Check logs for errors and retry logic  
• 	Confirm compliance disclaimers are visible in UI and LICENSE  
6. Compliance and Documentation  
• 	Add  with indemnification clause  
• 	Add  for contributors  
• 	Add  for end-users  
• 	Document setup and usage in   
7. Deployment  
• 	Deploy  to a StartOS instance (Raspberry Pi, x86 server, or VM)  
• 	Verify service runs correctly in StartOS dashboard  
• 	Share package with community or sideload privately  
