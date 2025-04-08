# rpg-framework

rpg-framework is an open-source, modular, text-based role-playing game (RPG) designed for both single-player and multiplayer experiences. The game is built on a microservices architecture using Docker containers for each component, making it highly extensible and easy to deploy. The game’s core system revolves around a central hub or nexus where players can gain experience points and explore different themed environments through "portals."

Key Features:
Modular World Design: The game is structured to support multiple modular regions/maps that can be easily added, including fantasy, technology, life-like, and hybrid themes. As players progress, they unlock portals that teleport them to new worlds with unique challenges.

Backend & Frontend Separation: The game utilizes a backend service written in Rust with PostgreSQL for data storage, and a React-based frontend that communicates with the backend via a RESTful API.

Dockerized Architecture: All components of the game, including the backend, frontend, PostgreSQL database, and CI/CD pipeline, are containerized using Docker. This ensures easy setup, scalability, and portability.

CI/CD Integration: Built-in Jenkins for continuous integration and deployment, coupled with GitHub/Gitea repositories for version control, allows for automated build and release processes.

Authentication: The game integrates with Keycloak for secure user authentication, with role-based access control (RBAC) to manage player sessions and game features.

Grafana Monitoring: The project includes Grafana integration for real-time monitoring and visualization of game-related metrics, server health, and performance.

Health Checks: System health and database connectivity are continuously monitored through custom health check endpoints, ensuring optimal performance.

Technologies Used:
Backend: Rust (Actix Web, Axum), PostgreSQL (for data storage)

Frontend: React.js, Webpack

Authentication: Keycloak

CI/CD: Jenkins, GitHub/Gitea

Containerization: Docker, Docker Compose

Monitoring: Grafana

This project is open-sourced, allowing contributors to expand the game by adding new worlds, levels, dungeons, character classes, and skills. We encourage community involvement to help grow the game's features and provide new content!
