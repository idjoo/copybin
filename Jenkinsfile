pipeline {
    agent any

    stages {
        stage('Format') {
            steps {
                echo '[+] Formatting...'
                sh 'cargo fmt --all -- --check'
            }
        }
    
        stage('Lint') {
            steps {
                echo '[+] Linting...'
                sh 'cargo clippy --all -- -D warnings'
            }
        }

        stage('Build') {
            steps {
                echo '[+] Building...'
                sh   'cargo build'
            }
        }
    }
}
