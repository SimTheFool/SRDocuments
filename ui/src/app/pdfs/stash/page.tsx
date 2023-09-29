import { MasonryGrid } from "@/components/MasonryGrid";
import { Space } from "@/components/Space";
import { TitleSection } from "@/components/TitleSection";
import { Drone } from "@/components/items/Drone";
import { ItemCard } from "@/components/items/ItemCard";
import { Tech } from "@/components/items/Tech";
import { Weapon } from "@/components/items/Weapon";
import { Box } from "@radix-ui/themes";
import { characters } from "resources";
import { A4Format } from "../A4Format";

const shrimp = characters.shrimp;

export default function Home() {
  return (
    <A4Format border>
      <Box pt={"2"}>
        <MasonryGrid columns={3}>
          <Box>
            <TitleSection>Réserve</TitleSection>
            <Space />
          </Box>
          {Object.entries(shrimp.tech || {}).map(([name, tech]) => {
            return (
              <Box pb={"4"} pr={"2"}>
                <Box>Départ</Box>
                <Box>Courant</Box>
                <Tech tech={tech} name={name} />
              </Box>
            );
          })}
        </MasonryGrid>
      </Box>
    </A4Format>
  );
}
