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
import { BaseAction } from "@/components/actions/BaseAction";

const shrimp = characters.shrimp;

export default function Home() {
  return (
    <A4Format border>
      <Box pt={"2"}>
        <MasonryGrid columns={3}>
          <Box>
            <TitleSection>Inventory</TitleSection>
            <Space />
          </Box>
          {Object.entries(shrimp.drones || {}).map(([name, drone]) => {
            return (
              <Box pb={"4"} pr={"2"}>
                <Drone item={drone} name={name} />
              </Box>
            );
          })}
          {Object.entries(shrimp.weapons || {}).map(([name, weapon]) => {
            return (
              <Box pb={"4"} pr={"2"}>
                <Weapon weapon={weapon} name={name} />
              </Box>
            );
          })}
          {Object.entries(shrimp.outfits || {}).map(([name, outfit]) => {
            return (
              <Box pb={"4"} pr={"2"}>
                <ItemCard item={outfit} name={name}>
                  {{
                    bottom: (
                      <>
                        {Object.entries(outfit.actions || {}).map(
                          ([name, action]) => (
                            <BaseAction name={name} action={action} />
                          )
                        )}
                      </>
                    ),
                  }}
                </ItemCard>
              </Box>
            );
          })}
          {Object.entries(shrimp.tech || {}).map(([name, tech]) => {
            return (
              <Box pb={"4"} pr={"2"}>
                <Tech tech={tech} name={name} />
              </Box>
            );
          })}
        </MasonryGrid>
      </Box>
    </A4Format>
  );
}
