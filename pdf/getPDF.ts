import puppeteer from "puppeteer";
import express from "express";
import next from "next";
import axios from "axios";
import path from "path";
import { PDFDocument } from "pdf-lib";
import fs from "fs/promises";
import fsSync from "fs";

type PdfsData = {
  characterSheets: {
    sheets: Record<string, string[]>;
    metadata: {
      pdfsConfig: {
        size: { width: number; height: number };
      };
    };
  };
};

const launchNextServer = async (port: number) => {
  const expressServer = express();
  const nextApp = next({
    dev: process.env.NODE_ENV !== "production",
    dir: "../ui",
  });
  const handleNextReq = nextApp.getRequestHandler();

  return new Promise<URL>((resolve) => {
    nextApp.prepare().then(() => {
      expressServer.all("*", (req: any, res: any) => {
        return handleNextReq(req, res);
      });

      expressServer.listen(port, () => {
        console.log(`> Ready on http://localhost:${port}`);
        const url = new URL("/", `http://localhost:${port}`);
        resolve(url);
      });
    });
  });
};

const main = async () => {
  const baseUrl = await launchNextServer(3002);

  const {
    characterSheets: { metadata, sheets },
  } = await axios
    .get(new URL("/pdfs", baseUrl).toString())
    .then((res) => res.data as PdfsData);

  const browser = await puppeteer.launch({
    headless: "new",
    defaultViewport: {
      width: metadata.pdfsConfig.size.width,
      height: metadata.pdfsConfig.size.height,
    },
  });

  const getSheet = async (endpoints: string[]) => {
    const page = await browser.newPage();
    const pdf = await PDFDocument.create();

    for (const endpoint of endpoints) {
      const url = new URL(endpoint, baseUrl);
      await page.goto(url.toString());
      await page.waitForSelector("h2");
      const data = await page.pdf({
        width: metadata.pdfsConfig.size.width,
        height: metadata.pdfsConfig.size.height,
      });
      const pdfPage = await PDFDocument.load(data);
      (await pdf.copyPages(pdfPage, pdfPage.getPageIndices())).forEach(
        (page) => {
          pdf.addPage(page);
        }
      );
    }

    return await pdf.save();
  };

  const pdfDirectory = path.join(__dirname, "dist");
  if (!fsSync.existsSync(pdfDirectory)) {
    await fs.mkdir(pdfDirectory, { recursive: true });
  }

  for (const charName of Object.keys(sheets)) {
    const endpoints = sheets[charName];
    const buf = await getSheet(endpoints);
    const pdfPath = path.join(pdfDirectory, `${charName}.pdf`);
    await fs.writeFile(pdfPath, buf);
  }

  await browser.close();
  process.exit(0);
};

main();
