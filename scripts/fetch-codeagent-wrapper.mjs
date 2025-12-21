import fs from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';

const REPO = 'cexll/myclaude';
const API = `https://api.github.com/repos/${REPO}/releases/latest`;

function detectAssetName(platform, arch) {
  // myclaude release assets:
  // - codeagent-wrapper-windows-amd64.exe
  // - codeagent-wrapper-windows-arm64.exe
  // - codeagent-wrapper-linux-amd64
  // - codeagent-wrapper-linux-arm64
  // - codeagent-wrapper-darwin-amd64
  // - codeagent-wrapper-darwin-arm64
  const isWin = platform === 'win32';
  const isMac = platform === 'darwin';
  const isLinux = platform === 'linux';

  const cpu = arch === 'x64' ? 'amd64' : arch === 'arm64' ? 'arm64' : null;
  if (!cpu) return null;

  if (isWin) return `codeagent-wrapper-windows-${cpu}.exe`;
  if (isMac) return `codeagent-wrapper-darwin-${cpu}`;
  if (isLinux) return `codeagent-wrapper-linux-${cpu}`;
  return null;
}

async function httpJson(url) {
  const res = await fetch(url, {
    headers: {
      'Accept': 'application/vnd.github+json',
      'User-Agent': 'code-ai-assistant-fetcher',
    },
  });
  if (!res.ok) {
    throw new Error(`HTTP ${res.status} ${res.statusText} for ${url}`);
  }
  return res.json();
}

async function httpDownload(url) {
  const res = await fetch(url, {
    headers: {
      'User-Agent': 'code-ai-assistant-fetcher',
    },
  });
  if (!res.ok) {
    throw new Error(`HTTP ${res.status} ${res.statusText} for ${url}`);
  }
  const arrayBuffer = await res.arrayBuffer();
  return Buffer.from(arrayBuffer);
}

async function main() {
  const args = process.argv.slice(2);
  const outDirArg = args.find((a) => a.startsWith('--outDir='))?.split('=')[1];
  const assetArg = args.find((a) => a.startsWith('--asset='))?.split('=')[1];

  const platform = process.platform;
  const arch = process.arch;

  const assetName = assetArg ?? detectAssetName(platform, arch);
  if (!assetName) {
    throw new Error(
      `无法为当前平台选择资产 (platform=${platform}, arch=${arch}). 你可以显式传入 --asset=<asset-name>。`
    );
  }

  const outDir = outDirArg
    ? path.resolve(outDirArg)
    : path.resolve('src-tauri', 'bin');

  const release = await httpJson(API);
  const assets = Array.isArray(release.assets) ? release.assets : [];
  const asset = assets.find((a) => a && a.name === assetName);

  if (!asset?.browser_download_url) {
    const available = assets.map((a) => a?.name).filter(Boolean).join('\n');
    throw new Error(
      `找不到资产 ${assetName}。\n可用资产:\n${available || '(none)'}\nAPI: ${API}`
    );
  }

  await fs.mkdir(outDir, { recursive: true });

  const outName = platform === 'win32' ? 'codeagent-wrapper.exe' : 'codeagent-wrapper';
  const outPath = path.join(outDir, outName);

  const bin = await httpDownload(asset.browser_download_url);
  await fs.writeFile(outPath, bin);

  if (platform !== 'win32') {
    await fs.chmod(outPath, 0o755);
  }

  process.stdout.write(
    `Downloaded ${assetName}\n -> ${outPath}\nTag: ${release.tag_name || 'unknown'}\n`
  );
}

main().catch((err) => {
  process.stderr.write(String(err?.stack || err) + '\n');
  process.exit(1);
});
