# RPG Framework

This project is a text-based role-playing game (RPG) framework designed to be modular, extendable, and easily deployable. The game includes various game mechanics such as item management, world generation, character creation, and backend services for multi-user support. The project also provides infrastructure for transitioning to graphical user interfaces in the future.

## Current Status
- **Backend**: Written in Rust using Axum and SQLx, this handles game data (e.g., world maps, characters, items) and connects to a PostgreSQL database.
- **Frontend**: Currently delivering a text-based interface using React, styled with the Catppuccin Mocha color scheme. It supports basic user interactions and backend connectivity. It is designed to transition to a graphical interface as the game evolves.
- **Database**: PostgreSQL is used for storing game data like character profiles, items, and world regions.
- **CI/CD**: Integrated Jenkins pipeline with Docker support, leveraging Traefik as a reverse proxy and Keycloak for authentication.

### Core Features
- **Backend**:
  - Modular game data structures such as characters, items, and regions.
  - RESTful API built with Axum for interacting with game data.
  - Database health checks, seed data setup, and migration management.
- **Frontend**:
  - Interactive, console-like interface with Catppuccin Mocha styling.
  - User authentication through Keycloak.
  - Ability for users to sign up and create characters, and for administrators to manage items and worlds.
- **Item System**:
  - Inventory management with properties like item durability, magical status, and cursed attributes.
  - Functionality for items like weapons, armor, and artifacts.

## Requirements

### Backend
- Rust (v1.75 or newer)
- PostgreSQL
- Cargo (Rust package manager)
- Docker (for development and deployment)

### Frontend
- Node.js (v16 or newer)
- npm (Node package manager)

### Docker
- Docker Compose for local development and deployment.

## Project Setup

### Clone the Repository
```bash
git clone https://github.com/sbarnabo/rpg-framework.git
cd rpg-framework
```

### Setting up the Backend

1. **Install Dependencies**:
   - Make sure Rust is installed. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/tools/install).
   - Install dependencies with Cargo:
     ```bash
     cd backend
     cargo build
     ```

2. **Configure PostgreSQL**:
   - You can either set up a local PostgreSQL instance or use Docker to run the database.
   - The `docker-compose.yml` file provides a service for PostgreSQL.

3. **Environment Configuration**:
   - Create a `.env` file in the backend directory with your PostgreSQL connection string:
     ```
     DATABASE_URL=postgres://user:password@localhost/rpg
     ```

4. **Running the Backend**:
   ```bash
   cargo run
   ```

### Setting up the Frontend

1. **Install Dependencies**:
   ```bash
   cd frontend
   npm install
   ```

2. **Run the Frontend**:
   ```bash
   npm run dev
   ```

   The frontend will be available at `http://localhost:3000`.

### Running with Docker

To run the whole application with Docker Compose, including the backend, frontend, and PostgreSQL:

```bash
docker-compose up -d
```

### Accessing the Services
- **Frontend**: `http://game.barnabo-connect.com`
- **Backend API**: `http://api.barnabo-connect.com`
- **Jenkins**: `http://localhost:8080` (for CI/CD management)

### Testing the Health Endpoint
Once the services are running, you can test the health check for the backend API by visiting:

```bash
curl http://localhost:8080/health
```

### Authentication via Keycloak
- Keycloak is integrated for user authentication. You can access the login page through the frontend application. Users must log in to create characters, while admins have additional privileges to manage items and worlds.

## Future Development

- **Graphical Interface**: Transition the frontend from a text-based interface to a graphical user interface (GUI) in future versions.
- **Expanded World Generation**: Implement more complex world-building mechanics and dungeons.
- **User Management**: Enhance the Keycloak integration for better user roles and permissions.

## Contributing

Contributions are welcome! Please fork the repository and create a pull request for any improvements or new features.

- **Bug Reports**: Please create issues for bugs or improvements.
- **Feature Requests**: If you have ideas for new features, please open a feature request issue.

## License

This project is licensed under the MIT License.
