import { MasonryGrid } from "@/components/MasonryGrid";
import { ParagraphStandard } from "@/components/ParagraphStandard";
import { Space } from "@/components/Space";
import { TitleSection } from "@/components/TitleSection";
import { ItemCard } from "@/components/items/ItemCard";
import { Slot } from "@/components/items/Slot";
import { Box, Flex } from "@radix-ui/themes";
import { characters } from "resources";
import { PdfContainer } from "../../PdfContainer";

type Props = {
  params: {
    name: string;
  };
};

export default function Page({ params: { name } }: Props) {
  const char = characters[name];

  return (
    <PdfContainer>
      <Box pt={"2"}>
        <MasonryGrid columns={4}>
          <Box>
            <TitleSection>Consommables et outils</TitleSection>
            <Space />
          </Box>
          {Object.entries(char.other || {}).map(([name, item]) => {
            return (
              <Box pb={"4"} pr={"1"} key={name}>
                <ItemCard item={item} name={name} noHand />
                <Flex pt={"1"}>
                  <ParagraphStandard>Restant:</ParagraphStandard>
                </Flex>
              </Box>
            );
          })}
        </MasonryGrid>
        <Box px={"2"}>
          <Box>
            <TitleSection>Stockage de données</TitleSection>
            <Space />
          </Box>
          <Slot size="XL" />
        </Box>
      </Box>
    </PdfContainer>
  );
}
