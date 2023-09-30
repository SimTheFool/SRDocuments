import { MasonryGrid } from "@/components/MasonryGrid";
import { Space } from "@/components/Space";
import { TitleSection } from "@/components/TitleSection";
import { Drone } from "@/components/items/Drone";
import { ItemCard } from "@/components/items/ItemCard";
import { Tech } from "@/components/items/Tech";
import { Weapon } from "@/components/items/Weapon";
import { Box, Flex } from "@radix-ui/themes";
import { characters } from "resources";
import { A4Format } from "../A4Format";
import { ParagraphStandard } from "@/components/ParagraphStandard";
import { Slot } from "@/components/items/Slot";

const shrimp = characters.shrimp;

export default function Home() {
  return (
    <A4Format border>
      <Box pt={"2"}>
        <MasonryGrid columns={4}>
          <Box>
            <TitleSection>Consommables et outils</TitleSection>
            <Space />
          </Box>
          {Object.entries(shrimp.other || {}).map(([name, item]) => {
            return (
              <Box pb={"4"} pr={"1"}>
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
    </A4Format>
  );
}
