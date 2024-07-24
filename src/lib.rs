pub mod utils {
    pub fn get_web_nginx_config_file<'a>(
        domain: &'a str,
        ssl_fullchain_path: &'a str,
        ssl_pem_path: &'a str,
        website_dist_path: &'a str,
    ) -> String {
        // https://medium.com/@kornchotpitakkul/deploy-a-node-js-and-vue-js-with-nginx-ssl-on-ubuntu-465f31216dc9
        format!(
            r#"
            server {{
                 listen      80;
                 listen      [::]:80;
                 server_name {domain} www.{domain};
                 return 301  https://$server_name$request_uri;
            }}
            server {{
                 listen       443 ssl http2;
                 listen       [::]:443 ssl http2;
                 server_name  {domain} www.{domain};
                 ssl_certificate {ssl_fullchain_path};
                 ssl_certificate_key {ssl_pem_path};

                 location / {{
                      root   {website_dist_path};
                      index  index.html;
                      try_files $uri $uri/ /index.html;
                 }}
                 error_page  500 502 503 504  /50x.html;
                 location = /50x.html {{
                      root   /usr/share/nginx/html;
                 }}
            }}
            "#
        )
    }

    pub fn get_ethereum_nginx_config_file<'a>(port: &'a i32, domain: &'a str) -> String {
        format!(
            r#"
            server {{
              listen {port};
              listen [::]:{port};
              server_name {domain} www.{domain};

              location ^~ /ws {{
                proxy_http_version 1.1;
                proxy_set_header Upgrade $http_upgrade;
                proxy_set_header Connection "upgrade";
                proxy_set_header X-Real-IP $remote_addr;
                proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
                proxy_set_header Host $http_host;
                proxy_set_header X-NginX-Proxy true;
                proxy_pass http://127.0.0.1:8546/;
              }}

              location ^~ /rpc {{
                proxy_http_version 1.1;
                proxy_set_header Upgrade $http_upgrade;
                proxy_set_header Connection "upgrade";
                proxy_set_header X-Real-IP $remote_addr;
                proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
                proxy_set_header Host $http_host;
                proxy_set_header X-NginX-Proxy true;
                proxy_pass http://127.0.0.1:8545/;
              }}
            }}
            "#
        )
    }

    pub fn get_startnode_command<'a>(
        newtork_id: &'a i32,
        http_address_ip: &'a str,
        ext_ip: &'a str,
        address: &'a str,
        ws_address_ip: &'a str,
    ) -> String {
        format!(
            r#"geth --networkid {newtork_id}  --datadir data --nodiscover --http --http.port "8545"  --port "30303" --http.addr "{http_address_ip}"  --http.corsdomain "*" --nat any --http.api "eth,web3,personal,net,miner,admin" --http.vhosts "*" --nat extip:{ext_ip}  --unlock '{address}' --password './password.sec'  --mine --miner.threads 4  --ipcpath "./data/geth.ipc" --allow-insecure-unlock --miner.etherbase '{address}' --miner.gasprice 1  --syncmode full --ws --ws.addr "{ws_address_ip}"  --ws.api "eth,net,web3,admin" --ws.origins "*""#
        )
    }

    pub fn get_genesis_file<'a>(address: &'a str, chain_id: &'a i32) -> String {
        format!(
            r#"
            {{
              "config": {{
                "chainId": {chain_id},
                "homesteadBlock": 0,
                "eip150Block": 0,
                "eip155Block": 0,
                "eip158Block": 0,
                "byzantiumBlock": 0,
                "constantinopleBlock": 0,
                "petersburgBlock": 0,
                "istanbulBlock": 0,
                "berlinBlock": 0,
                "clique": {{
                  "period": 1,
                  "epoch": 30000
                }}
              }},
              "difficulty": "1",
              "gasLimit": "8000000",
              "extradata": "0x0000000000000000000000000000000000000000000000000000000000000000{address}0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
              "alloc": {{
                "{address}": {{ "balance": "300000000" }},
                "f41c74c9ae680c1aa78f42e5647a62f353b7bdde": {{ "balance": "40000000" }}
              }}
            }}
           "#,
            address = address,
            chain_id = chain_id
        )
    }
}
