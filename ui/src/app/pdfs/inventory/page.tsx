import { MasonryGrid } from "@/components/MasonryGrid";
import { Space } from "@/components/Space";
import { TitleSection } from "@/components/TitleSection";
import { Drone } from "@/components/items/Drone";
import { Box, Flex } from "@radix-ui/themes";
import { Character, characters } from "resources";
import { A4Format } from "../A4Format";
import { ItemCard } from "@/components/items/ItemCard";
import { Card } from "@/components/Card";
import { TitleMin } from "@/components/TitleMin";
import { ParagraphStandard } from "@/components/ParagraphStandard";
import { MajorAction, MinorAction } from "@/components/Icons/Actions";

const shrimp = characters.shrimp;
const weapons = shrimp.weapons;

type LoadAction = Exclude<
  Exclude<Weapon["actions"], undefined>["recharger"],
  undefined
>;
const LoadAction = ({ action }: { action: LoadAction }) => {
  return (
    <Card>
      <Flex justify={"between"}>
        <Box>
          <TitleMin title={"Recharger"} />
          <Space />
          {action.description && (
            <ParagraphStandard>{action.description}</ParagraphStandard>
          )}
          <ParagraphStandard>_____/{action.ammo}</ParagraphStandard>
        </Box>
        <Box>
          <MajorAction />
        </Box>
      </Flex>
    </Card>
  );
};

type ShootAction = Exclude<
  Exclude<Weapon["actions"], undefined>["tir"],
  undefined
>;
const ShootAction = ({
  action: {
    damage,
    damage_type,
    major,
    minor,
    description,
    ranges,
    ammo_consumption,
  },
}: {
  action: ShootAction;
}) => {
  return (
    <Card>
      <Flex justify={"between"}>
        <Box>
          <TitleMin title={"Tir"} />
          <Space />
          <TitleMin
            subtitle={`${ranges.contact}/${ranges.near}/${ranges.short}/${ranges.medium}/${ranges.far}`}
          />
          <Space />
          {!!damage && (
            <TitleMin subtitle={<>VD: {`${damage} ${damage_type}`}</>} />
          )}
          <Space />
          {description && <ParagraphStandard>{description}</ParagraphStandard>}
        </Box>
        <Box>
          {Array.from({ length: major }).map((_, i) => (
            <MajorAction key={i} />
          ))}
          {Array.from({ length: minor }).map((_, i) => (
            <MinorAction key={i} />
          ))}
        </Box>
      </Flex>
    </Card>
  );
};

type Weapon = Exclude<Character["weapons"], undefined>[string];
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
            {tir && <ShootAction action={tir} />}
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
          {Object.entries(weapons || {}).map(([name, weapon]) => {
            return (
              <Box pb={"4"} pr={"2"}>
                <Weapon weapon={weapon} name={name} />
              </Box>
            );
          })}
        </MasonryGrid>
      </Box>
      {/* <Grid
        columns="2"
        gap="2"
        style={{
          gridTemplateColumns: "58% 42%",
          gridTemplateRows: "1fr",
        }}
        pt={"1"}
        px={"2"}
      >
        <Box>
          <Stats char={shrimp} />
          <Resources char={shrimp} />
          <Monitors char={shrimp} />
        </Box>

        <Box>
          <Box
            pl={"2"}
            style={{
              borderLeft: "2px solid var(--gray-10)",
            }}
          >
            <Identities char={shrimp} />
          </Box>
          <Box
            pl={"2"}
            style={{
              borderLeft: "2px solid var(--gray-10)",
            }}
          >
            <Skills char={shrimp} />
          </Box>
        </Box>
      </Grid> */}
      {/* <Box
        pt={"3"}
        mx={"3"}
        style={{
          borderTop: "2px solid var(--gray-10)",
        }}
      >
        <Effects char={shrimp} />
      </Box>
      <Box
        pt={"3"}
        mx={"3"}
        style={{
          borderTop: "2px solid var(--gray-10)",
        }}
      >
        <Section title={<TitleSection>Notes</TitleSection>}></Section>
      </Box> */}
    </A4Format>
  );
}
