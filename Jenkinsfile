pipeline {
  agent any
  stages {
    stage('prepare') {
      parallel {
        stage('create manpage') {
          agent {
            node {
              label 'nix'
            }

          }
          steps {
            sh 'env'
            sh 'nix-build -A manpage'
            stash name: 'free.1', includes: 'result/free.1'
          }
        }
        stage('create manifest') {
          agent {
            node {
              label 'nix'
            }

          }
          steps {
            sh 'nix-build -A manifest'
            stash name: 'manifest', includes: 'result/manifest'
          }
        }
      }
    }
    stage('build / test release') {
      agent {
        node {
          label 'freebsd'
        }

      }
      steps {
        sh 'cargo test --release'
        unstash 'free.1'
        unstash 'manifest'

        sh '''
          mkdir -p usr/local/bin
          cp -v target/release/free usr/local/bin

          mkdir -p usr/local/man/man1
          cp -v result/free.1 usr/local/man/man1


          pkg create -v -r . -M result/manifest
          ls -ltr
        '''

        archiveArtifacts artifacts: 'free*txz'
      }
    }
  }
}