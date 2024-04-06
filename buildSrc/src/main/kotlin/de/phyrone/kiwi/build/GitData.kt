package de.phyrone.kiwi.build

import org.eclipse.jgit.api.Git
import org.eclipse.jgit.revwalk.RevCommit
import java.io.File

class GitData(rootDir: File) {
    val git = Git.open(rootDir)

    fun branch(): String {
        return git.repository.branch
    }

    fun lastCommit(): RevCommit = git.log().setMaxCount(1)
        .call().firstOrNull() ?: error("could not find git history")

}