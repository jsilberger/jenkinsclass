pipeline {
    agent any

    options {
        timestamps()
        disableConcurrentBuilds()
        timeout(time: 60, unit: 'MINUTES')
    }

    environment {
        CARGO_HOME = "${WORKSPACE}/.cargo"
        CARGO_TARGET_DIR = "${WORKSPACE}/target"
        RUSTUP_HOME = "${WORKSPACE}/.rustup"
        PATH = "${WORKSPACE}/.cargo/bin:${env.PATH}"
    }

    stages {
        stage('Rust Toolchain') {
            steps {
                sh '''
                    set -eu
                   
                        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
                   
                    . "${CARGO_HOME}/env"
                    "${CARGO_HOME}/bin/rustup" default stable
                    "${CARGO_HOME}/bin/rustup" component add clippy rustfmt
                    rustup component add clippy rustfmt
                    . "${CARGO_HOME}/env"
                    
                    cargo --version
                    rustc --version
                '''
            }
        }

        stage('Format Check') {
            steps {
                sh '''
                    . "$HOME/.cargo/env"
                    cargo fmt --all -- --check
                '''
            }
        }

        stage('Clippy') {
            steps {
                sh '''
                    . "$HOME/.cargo/env"
                    cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic
                '''
            }
        }

        stage('Test') {
            steps {
                sh '''
                    . "$HOME/.cargo/env"
                    cargo test --all-features
                '''
            }
        }

        stage('Release Build') {
            steps {
                sh '''
                    . "$HOME/.cargo/env"
                    cargo build --release --all-targets
                '''
            }
        }

        stage('Deployment') {
               steps {
                sh 'touch release_deploy_marker.txt'
                sh 'echo deployed'
            }
        }
    }

    post {
        always {
            archiveArtifacts artifacts: 'target/**/deps/*', allowEmptyArchive: true
            echo "Build finished for ${env.JOB_NAME} #${env.BUILD_NUMBER}."
        }
        success {
            echo 'Pipeline succeeded.'
        }
        failure {
            echo 'Pipeline failed. Check stage logs above for details.'
        }
    }
}
