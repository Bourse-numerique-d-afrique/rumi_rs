// instlal rust on the new server
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Step 2: Configure DNS for Your Domain
Log in to Your Domain Registrar's Website
Find the DNS Management Section
Add an A Record
Name: @ (or your desired subdomain, e.g., www)
Type: A
Value: Your server's public IP address
TTL: Default or 3600


install Cerbot
Certbot is a tool to obtain Let's Encrypt certificates. Install it on your server:
```
sudo apt-get update
sudo apt-get install certbot
```

Obtain the Certificate
Run Certbot with the standalone option:
```
sudo certbot certonly --standalone -d yourdomain.com -d www.yourdomain.com
```

Follow the instructions to complete the process. The certificates will be stored in /etc/letsencrypt/live/yourdomain.com/.

Step 4: Update the Paths to Your SSL Certificate and Private Key
Modify the paths in your main.rs to match the locations of your SSL certificate and private key:

```
let cert_file = File::open("/etc/letsencrypt/live/yourdomain.com/fullchain.pem").expect("cannot open certificate file");
let key_file = File::open("/etc/letsencrypt/live/yourdomain.com/privkey.pem").expect("cannot open private key file");

```


Step 5: Open Ports on Your Server
Ensure your server's firewall allows traffic on ports 80 (HTTP) and 443 (HTTPS).
```
sudo ufw allow 80
sudo ufw allow 443
```
