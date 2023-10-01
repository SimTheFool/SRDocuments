import { PDFDocument } from "pdf-lib";
import puppeteer from "puppeteer";
import { PDFsMetadata } from "./startUI";

export const getPDF = async (
  endpoints: string[],
  baseUrl: URL,
  metadata: PDFsMetadata
) => {
  const browser = await puppeteer.launch({
    headless: "new",
    defaultViewport: {
      width: metadata.size.width,
      height: metadata.size.height,
    },
  });

  const page = await browser.newPage();
  const pdf = await PDFDocument.create();

  for (const endpoint of endpoints) {
    const url = new URL(endpoint, baseUrl);
    await page.goto(url.toString());
    await page.waitForSelector("h2");
    const data = await page.pdf({
      width: metadata.size.width,
      height: metadata.size.height,
    });
    const pdfPage = await PDFDocument.load(data);
    (await pdf.copyPages(pdfPage, pdfPage.getPageIndices())).forEach((page) => {
      pdf.addPage(page);
    });
  }

  await browser.close();
  return await pdf.save();
};
