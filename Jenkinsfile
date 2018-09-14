// FIXME: move this in a jenkins-plugin
import org.jenkinsci.plugins.pipeline.modeldefinition.Utils

final REPO_URL = "http://192.168.1.3/free.git"

node("nix") {
    stage("prepare"){
        git REPO_URL
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

    stage("setup env") {
        deleteDir()
        git REPO_URL

        env.FREE_VERSION_CARGO = {
            def txt = readFile("Cargo.toml")
            def group = (txt =~ /version\s*=\s*"([\d\.]+)"/)
            group[0][1]
        }()

        env.FREE_VERSION_GIT = sh(returnStdout: true, script: "git describe --always --tags").trim()
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

        echo "sign package"
        sign(FREE_PACKAGE, GPG_PASSPHRASE)

        echo "archive artifacts"
        archiveArtifacts artifacts: "${FREE_PACKAGE}, ${FREE_PACKAGE}.sig"
    }


    stage("dogfooding") {
        if(env.NODE_NAME == "wurzel") {
            sh "sudo pkg install -f -y ${env.FREE_PACKAGE}"
            sh "free -V"
            sh "free -ha"
        } else {
            echo "skipped - run's only on 'wurzel"
            // FIXME: move this in a jenkins-plugin
            Utils.markStageSkippedForConditional(STAGE_NAME)
        }
    }


    stage("publish"){
        echo "versions - cargo: '${env.FREE_VERSION_CARGO}', git: '${env.FREE_VERSION_GIT}'"
        if(env.FREE_VERSION_CARGO == env.FREE_VERSION_GIT) {
            echo "new release - publish!"

            def ghUtils = new GitHubReleaseUtils(
                owner: "j-keck",
                repo: "free",
                token: env.GITHUB_TOKEN,
                script: this
            )

            ghUtils.delete(env.FREE_VERSION_CARGO)
            ghUtils.create(env.FREE_VERSION_CARGO)

            ghUtils.uploadAsset(env.FREE_VERSION_CARGO,
                                env.FREE_PACKAGE,
                                "application/binary"
            )
            ghUtils.uploadAsset(env.FREE_VERSION_CARGO,
                                "${env.FREE_PACKAGE}.sig",
                                "application/binary"
            )


             currentBuild.description = "- release build -"
        } else {
            echo "skip publish"
            // FIXME: move this in a jenkins-plugin
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



def sign(String fileName, String passphrase) {
    String cmd = "gpg --pinentry-mode loopback --passphrase ${passphrase} " +
                 " --detach-sign --output ${fileName}.sig ${fileName}"

    echo "write signature in ${fileName}.sig"
    // execute without logging
    sh "#!/bin/sh -e\n${cmd}"
}


// FIXME: move this in a jenkins-plugin
//
// usage: new GitHubReleaseUtils(.., script: this)
class GitHubReleaseUtils {

    String owner
    String repo
    String token
    Script script

    final GH_URL = "https://api.github.com"

    def create(String tag) {
        script.echo("create new gh release with tag: ${tag}")
        def data = """{"tag_name": "${tag}"}"""
        def url = "${GH_URL}/repos/${owner}/${repo}/releases".toURL()
        def con = url.openConnection()
        con.setRequestMethod("POST")
        con.setDoOutput(true)
        con.setRequestProperty("Authorization", "token ${token}")
        sendText(con, data)
        con.connect()
        script.echo(con.content.text)
    }

    def delete(String tag){
        script.echo("delete gh release with tag: ${tag}")
        try {
            def release = lookupReleaseByTag(tag)

            def url = "${GH_URL}/repos/${owner}/${repo}/releases/${release.id}".toURL()
            def con = url.openConnection()
            con.setRequestMethod("DELETE")
            con.setRequestProperty("Authorization", "token ${token}")
            con.connect()
            script.echo(con.content.text)
            script.echo("release deleted")
        } catch (all) {
            script.echo("release NOT delete - reason: ${all}")
        }
    }


    def uploadAsset(String tag, String path, String contentType) {
        script.echo("upload asset: ${path}")

        def release = lookupReleaseByTag(tag)

        String fileName = new File(path).name
        // BAD, BAD, BAD!!!
        def uploadUrl = release.upload_url
            .replaceFirst(/\{\?name,label\}/, "?name=${fileName}")
            .toURL()


        def con = uploadUrl.openConnection()
        con.setRequestMethod("POST")
        con.setDoOutput(true)
        con.setRequestProperty("Content-Type", contentType)
        con.setRequestProperty("Authorization", "token ${token}")
        sendFile(con, path)
        con.connect()
        script.echo(con.content.text)
    }


    def lookupReleaseByTag(String tag) {
        def url = "${GH_URL}/repos/${owner}/${repo}/releases/tags/${tag}".toURL()
        def con = url.openConnection()
        con.setRequestProperty("Authorization", "token ${token}")
        new groovy.json.JsonSlurper().parseText(con.content.text)
    }


    def sendText(java.net.URLConnection con, String data) {
        def writer = new OutputStreamWriter(con.outputStream)
        writer.write(data)
        writer.flush()
        writer.close()
    }


    def sendFile(java.net.URLConnection con, String path) {
        // this script run's in a groovy sandbox without access to the filesystem.
        // HACK: use the 'readFile' step to read the file as base64 encoded
        // string, convert it back and write / send the content.
        String content = script.readFile(file: path, encoding: "Base64")
        def stream = new BufferedOutputStream(con.outputStream)
        stream.write(content.decodeBase64())
        stream.flush()
        stream.close()
    }
}

