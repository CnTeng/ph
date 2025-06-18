# ph

A helper for [impermanence](https://github.com/nix-community/impermanence) and [preservation](https://github.com/nix-community/preservation).

When you remove entries from your persistence configuration, those entries are
not automatically deleted from the persistence root. This tool helps you prune
unused entries from your persistence roots.

## ✨ Features

#### Prune persistence roots

![prune](https://github.com/user-attachments/assets/b2a7a0de-d293-43c1-bd2f-3622d757cfff)

#### Persist entry

![persist](https://github.com/user-attachments/assets/28554852-3151-4193-abc4-78f71634e084)

## 🛠️ Usage

### NixOS

We provide a NixOS module. To enable it, set the following configuration:

```nix
{
  programs.ph.enable = true;
}
```

<!-- MARKDOWN LINKS & IMAGES -->
