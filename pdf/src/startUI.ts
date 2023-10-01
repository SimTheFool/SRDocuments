import axios from "axios";
import express from "express";
import next from "next";

export type PDFsMetadata = {
  size: { width: number; height: number };
};

export type PDFsSheetConfig = Record<string, string[]>;

export type PDFsConfigAndData = {
  characters: {
    sheets: PDFsSheetConfig;
    metadata: PDFsMetadata;
  };
};

export const startUI = async (port: number) => {
  const expressServer = express();
  const nextApp = next({
    dev: process.env.NODE_ENV !== "production",
    dir: "../ui",
  });
  const handleNextReq = nextApp.getRequestHandler();

  const uiUrl = await new Promise<URL>((resolve) => {
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

  const pdfsConfig = await axios
    .get(new URL("/pdfs", uiUrl).toString())
    .then((res) => res.data as PDFsConfigAndData);

  return [uiUrl, pdfsConfig] as const;
};
