import { MasonryGrid } from "@/components/MasonryGrid";
import { Space } from "@/components/Space";
import { TitleSection } from "@/components/TitleSection";
import { LoadAction } from "@/components/actions/LoadAction";
import { ShotAction } from "@/components/actions/ShotAction";
import { Drone } from "@/components/items/Drone";
import { ItemCard } from "@/components/items/ItemCard";
import { Box } from "@radix-ui/themes";
import { Weapon, characters, RangeLabels } from "resources";
import { A4Format } from "../A4Format";
import { Ruler } from "@/components/Ruler";
import { BaseAction } from "@/components/actions/BaseAction";

const shrimp = characters.shrimp;

const DistanceNbRuler = ({ distanceByNb }: { distanceByNb: RangeLabels }) => {
  const nbs = Object.entries(distanceByNb).map(([key]) => {
    const k = key.replace("r", "");
    const nb = parseInt(k);
    return nb;
  });
  const sortedNbs = nbs.sort();

  return <Ruler items={sortedNbs} />;
};

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
        inner: (
          <>
            {weapon.range_labels && (
              <DistanceNbRuler distanceByNb={weapon.range_labels} />
            )}
          </>
        ),
        bottom: (
          <>
            {Object.entries(otherActions).map(([name, action]) => (
              <BaseAction name={name} action={action} />
            ))}
            {recharger && <LoadAction action={recharger} />}
            {tir && (
              <ShotAction
                name={"Tir"}
                action={tir}
                rangeLabels={weapon.range_labels}
              />
            )}
            {tir_semi_auto && (
              <ShotAction
                name={"Tir semi"}
                action={tir_semi_auto}
                rangeLabels={weapon.range_labels}
              />
            )}
            {tir_rafale && (
              <ShotAction
                name={"Tir rafale"}
                action={tir_rafale}
                rangeLabels={weapon.range_labels}
              />
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
        <MasonryGrid columns={4}>
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
