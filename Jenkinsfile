pipeline {
  agent any

  environment {
    RUST_BACKTRACE = 1
  }

  stages {
    stage('Checkout') {
      steps {
        git 'http://gitea.barnabo-connect.com/your-org/rpg-project.git'
      }
    }
    stage('Build') {
      steps {
        sh 'cargo build --release'
      }
    }
    stage('Test') {
      steps {
        sh 'cargo test'
      }
    }
    stage('Docker Build & Push') {
      steps {
        sh 'docker buildx build --platform linux/arm64 -t your-registry/rpg-backend . --push'
      }
    }
    stage('Deploy') {
      steps {
        sh 'docker compose up -d'
      }
    }
  }
}
