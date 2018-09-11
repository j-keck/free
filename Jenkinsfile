import org.jenkinsci.plugins.pipeline.modeldefinition.Utils

node("nix") {
    checkout scm
    stage("prepare"){
        parallel(
            "create manpage": {
                sh "nix-build -A manpage"
                stash name: "free.1", includes: "result/free.1"
            },
            "create manifest": {
                sh "nix-build -A manifest"
                stash name: "manifest", includes: "result/manifest"
            }
        )
    }
}

node("freebsd") {

    checkout scm
    //
    // FIXME: Scripts not permitted to use method hudson.plugins.git.GitSCM getBranches
    //
    // `checkout scm` don't pulls tags, so i need this
    // checkout([
    //     $class: 'GitSCM',
    //     branches: scm.branches,
    //     doGenerateSubmoduleConfigurations: scm.doGenerateSubmoduleConfigurations,
    //     extensions: scm.extensions + [[$class: 'CloneOption', noTags: false, reference: '', shallow: true]],
    //     submoduleCfg: [],
    //     userRemoteConfigs: scm.userRemoteConfigs
    // ])


    stage("setup env") {
        env.FREE_VERSION_CARGO = {
            def txt = readFile("Cargo.toml")
            def group = (txt =~ /version\s*=\s*"([\d\.]+)"/)
            group[0][1]
        }()

        env.FREE_VERSION_GIT = sh(returnStdout: true, script: "git describe --always --tags")
        env.FREE_PACKAGE = "free-${env.FREE_VERSION_CARGO}.txz"

        // log the actual environemnt
        sh 'env|sort'
    }

    stage("test (release)") {
        cargo "test --release"
    }

    stage("clippy") {
        cargo "+nightly clippy"
    }


    stage("package"){
        echo "executable"
        cargo "build --release"
        sh "mkdir -p usr/local/bin"
        sh "cp -v target/release/free usr/local/bin"

        echo "manpage"
        unstash name: 'free.1'
        sh "mkdir -p usr/local/man/man1"
        sh "cp -v result/free.1 usr/local/man/man1"

        echo "create package"
        unstash name: "manifest"
        sh "pkg create -v -r . -M result/manifest"

        echo "archive artifacts"
        archiveArtifacts artifacts: "free*txz"
    }


    stage("dogfooding") {
        if(env.NODE_NAME == "wurzel") {
            sh "sudo pkg install -f -y ${env.FREE_PACKAGE}"
            sh "free -V"
            sh "free -ha"
        } else {
            echo "skipped - run's only on 'wurzel"
            Utils.markStageSkippedForConditional(STAGE_NAME)
        }
    }


    stage("publish"){
        if(env.FREE_VERSION_CARGO == env.FREE_VERSION_GIT) {
            // FIXME: publish to github
            //   - how to fetch git tags!?!?
        } else {
            Utils.markStageSkippedForConditional(STAGE_NAME)
        }
    }
}


def cargo(String args) {
    withEnv(["PATH=$HOME/.cargo/bin:$PATH"]) {
        sh "rustc --version"
        sh "cargo ${args}"
    }
}
