import puppeteer from "puppeteer";
import express from "express";
import next from "next";

const launchNextServer = async (port: number) => {
  const expressServer = express();
  const nextApp = next({
    dev: process.env.NODE_ENV !== "production",
    dir: "../ui",
  });
  const handleNextReq = nextApp.getRequestHandler();

  return new Promise<void>((resolve, reject) => {
    nextApp.prepare().then(() => {
      expressServer.all("*", (req: any, res: any) => {
        return handleNextReq(req, res);
      });

      expressServer.listen(port, () => {
        console.log(`> Ready on http://localhost:${port}`);
        resolve();
      });
    });
  });
};

const wait = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

const main = async () => {
  await launchNextServer(3002);

  const a4FromWidth = (width: number) => {
    const ratio = 1.414;
    const height = width * ratio;
    return { width: Math.floor(width), height: Math.floor(height) };
  };

  const browser = await puppeteer.launch({
    headless: false,
    defaultViewport: {
      ...a4FromWidth(2500),
    },
  });
  const page = await browser.newPage();
  await page.goto("http://localhost:3002/pdfs/summary");
  await page.waitForSelector("h1");

  await page.screenshot({
    fullPage: true,
    quality: 100,
    type: "jpeg",
    path: `${__dirname}/my-fance-invoice.jpeg`,
  });
  await browser.close();
  process.exit(0);
};

main();
