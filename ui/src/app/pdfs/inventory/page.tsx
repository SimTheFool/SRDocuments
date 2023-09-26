import { MasonryGrid } from "@/components/MasonryGrid";
import { Space } from "@/components/Space";
import { TitleSection } from "@/components/TitleSection";
import { LoadAction } from "@/components/actions/LoadAction";
import { ShotAction } from "@/components/actions/ShotAction";
import { Drone } from "@/components/items/Drone";
import { ItemCard } from "@/components/items/ItemCard";
import { Box } from "@radix-ui/themes";
import { Weapon, characters } from "resources";
import { A4Format } from "../A4Format";

const shrimp = characters.shrimp;

const Weapon = ({ weapon, name }: { weapon: Weapon; name: string }) => {
  const {
    recharger,
    attaquer,
    tir,
    tir_rafale,
    tir_semi_auto,
    ...otherActions
  } = weapon.actions || {};

  return (
    <ItemCard item={weapon} name={name}>
      {{
        bottom: (
          <>
            {recharger && <LoadAction action={recharger} />}
            {tir && (
              <ShotAction action={tir} rangeLabels={weapon.range_labels} />
            )}
          </>
        ),
      }}
    </ItemCard>
  );
};

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
        </MasonryGrid>
      </Box>
    </A4Format>
  );
}
