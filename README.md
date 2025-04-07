# rpg-framework

Specific to docker-compose.yml
Replace your-email@example.com with your real email for Let’s Encrypt.

DNS Setup: Point game.barnabo-connect.com and api.barnabo-connect.com to your Pi's public IP (via DDNS if needed).

The frontend is served at http://game.barnabo-connect.com, and the backend API at http://api.barnabo-connect.com.

Traefik dashboard: http://<pi-ip>:8080 (insecure dashboard, secure it in prod).

Jenkins dashboard: http://<pi-ip>:8081
