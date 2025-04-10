pipeline {
  agent any

  environment {
    RUST_BACKTRACE = 1
    IMAGE_TAG = 'latest'
  }

  stages {
    stage('Checkout') {
      steps {
        git branch: 'main', url: 'http://gitea.barnabo-connect.com/your-org/rpg-project.git'
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
        withCredentials([usernamePassword(credentialsId: 'docker-registry-creds', usernameVariable: 'DOCKER_USER', passwordVariable: 'DOCKER_PASS')]) {
          sh 'docker login -u $DOCKER_USER -p $DOCKER_PASS your-registry'
        }
        sh """
          docker buildx build \
            --platform linux/arm64 \
            -t your-registry/rpg-backend:${IMAGE_TAG} \
            --push .
        """
      }
    }

    stage('Deploy') {
      steps {
        sh 'docker compose up -d'
      }
    }
  }

  post {
    success {
      echo '✅ Deployment succeeded!'
    }
    failure {
      echo '❌ Build or deploy failed.'
    }
  }
}
