<p align="center">
  <h1 align="center">shfl</h1>
  <p align="center">
    <a href="https://github.com/dhth/shfl/actions/workflows/main.yml"><img alt="Build status" src="https://img.shields.io/github/actions/workflow/status/dhth/shfl/main.yml?style=flat-square"></a>
    <a href="https://github.com/dhth/shfl/releases/latest"><img alt="Latest Release" src="https://img.shields.io/github/release/dhth/shfl.svg?style=flat-square"></a>
    <a href="https://github.com/dhth/shfl/releases"><img alt="Commits Since Latest Release" src="https://img.shields.io/github/commits-since/dhth/shfl/latest?style=flat-square"></a>
  </p>
</p>

`shfl` (short for "shuffle") lets you easily rearrange lines in a file with
simple keymaps.

![demo](https://github.com/user-attachments/assets/07bd4b71-f78b-4b82-8080-3b973258bf55)

🤔 Motivation
---

I like switching my tmux sessions quickly (using `tmux switch-client -t
"$session"`, triggered via a shortcut). The session names are stored in a local
file, and I map a key to a session on a specific line number. To easily change
which session is assigned to which line number, I needed a tool that would start
up quickly and have easy keymaps to reorder lines in a file.

https://github.com/user-attachments/assets/58585b04-6474-4172-b6f8-40c75b486113

💾 Installation
---

**homebrew**:

```sh
brew install dhth/tap/shfl
```

**cargo**:

```sh
cargo install --git https://github.com/dhth/shfl.git
```

Or get the binaries directly from a [release][2]. Read more about verifying the
authenticity of released artifacts [here](#-verifying-release-artifacts).

⌨️ Keymaps
---

```
K                    move item one position above
Enter                move item/selection to the start of the list
j / Down             go down
k / Up               go up
[1-9]                move current item to index in list
g                    go to the start of the list
G                    go to the end of the list
w                    write to file
space / s            select/unselect item
?                    show/hide help view
Esc / q              go back/reset selection/exit
```

🔐 Verifying release artifacts
---

In case you get the `shfl` binary directly from a [release][2], you may want to
verify its authenticity. Checksums are applied to all released artifacts, and
the resulting checksum file is signed using
[cosign](https://docs.sigstore.dev/cosign/installation/).

Steps to verify (replace `A.B.C` in the cshflands listed below with the version
you want):

1. Download the following files from the release:

   ```text
   - shfl_A.B.C_checksums.txt
   - shfl_A.B.C_checksums.txt.pem
   - shfl_A.B.C_checksums.txt.sig
   ```

2. Verify the signature:

   ```shell
   cosign verify-blob shfl_A.B.C_checksums.txt \
       --certificate shfl_A.B.C_checksums.txt.pem \
       --signature shfl_A.B.C_checksums.txt.sig \
       --certificate-identity-regexp 'https://github\.com/dhth/shfl/\.github/workflows/.+' \
       --certificate-oidc-issuer "https://token.actions.githubusercontent.com"
   ```

3. Download the compressed archive you want, and validate its checksum:

   ```shell
   curl -sSLO https://github.com/dhth/shfl/releases/download/vA.B.C/shfl_A.B.C_linux_amd64.tar.gz
   sha256sum --ignore-missing -c shfl_A.B.C_checksums.txt
   ```

3. If checksum validation goes through, uncompress the archive:

   ```shell
   tar -xzf shfl_A.B.C_linux_amd64.tar.gz
   ./shfl
   ```

Acknowledgements
---

`shfl` is built using [ratatui][1].

[1]: https://github.com/ratatui/ratatui
[2]: https://github.com/dhth/shfl/releases
