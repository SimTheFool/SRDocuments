import { pdfsConfig } from "@/utils/config";
import { characters } from "resources";

const getCharacterSheet = (charName: string) => [
  `./pdfs/${charName}/summary`,
  `./pdfs/${charName}/powers`,
  `./pdfs/${charName}/inventory`,
];

export async function GET() {
  const charNames = Object.keys(characters);

  const charSheetsKV = charNames.map((charName) => [
    charName,
    getCharacterSheet(charName),
  ]);

  return Response.json({
    characters: {
      sheets: Object.fromEntries(charSheetsKV),
      metadata: {
        ...pdfsConfig,
      },
    },
  });
}
