pipeline {
    agent any

    stages {
        stage('Build') {
            steps {
                echo '----------Start----------'
                sh   'cargo build'
                echo '-----------End-----------'
                echo 'SUCCESS'
            }
        }
    }
}
