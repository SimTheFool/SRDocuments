import fsSync from "fs";
import fs from "fs/promises";
import path from "path";
import { getPDF } from "./getPDF";
import { startUI } from "./startUI";
import { parseArgs } from "node:util";

const main = async () => {
  const {
    values: { name },
  } = parseArgs({
    options: {
      name: {
        type: "string",
        short: "n",
      },
    },
  });

  const [uiUrl, conf] = await startUI(3002);

  const pdfDirectory = path.join(__dirname, "../dist");
  if (!fsSync.existsSync(pdfDirectory)) {
    await fs.mkdir(pdfDirectory, { recursive: true });
  }

  const promises = Object.keys(conf.characters.sheets).map(async (charName) => {
    if (name && charName !== name) return;
    const endpoints = conf.characters.sheets[charName];
    const buf = await getPDF(endpoints, uiUrl, conf.characters.metadata);
    const pdfPath = path.join(pdfDirectory, `${charName}.pdf`);
    await fs.writeFile(pdfPath, buf);
  });

  await Promise.all(promises);
  process.exit(0);
};

main();
