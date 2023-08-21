import { Card } from "@/components/Card";
import { FlexList } from "@/components/FlexList";
import { Section } from "@/components/Section";
import { TextStandard } from "@/components/TextStandard";
import { TitleMin } from "@/components/TitleMin";
import { Box } from "@radix-ui/themes";

type EffectsProps = {};

export const Effects = ({}: EffectsProps) => {
  return (
    <Section title="Effets" separator="left">
      <FlexList>
        <Box
          pr={"2"}
          grow={"1"}
          pb={"2"}
          style={{
            width: "50%",
          }}
        >
          <Card>
            <TitleMin title={"Bricoleur prévoyant"} subtitle={"Trait"} />
            <TextStandard>
              Vous avez -1 lorque vous utilisez une machine que vous avez
              bricolé
            </TextStandard>
          </Card>
        </Box>

        <Box
          pr={"2"}
          grow={"1"}
          pb={"2"}
          style={{
            width: "50%",
          }}
        >
          <Card>
            <TitleMin title={"Ami des sprites machine"} subtitle={"Trait"} />
            <TextStandard>
              Vous avez -1 lorque vous invoquzez un sprite machine
            </TextStandard>
          </Card>
        </Box>

        <Box pr={"2"} grow={"1"} pb={"2"}>
          <Card>
            <TitleMin title={"Paralysie du combat"} subtitle={"Trait"} />
            <TextStandard>
              Le 1er tour d'un affrontement, vous ne pouvez pas vous déplacer et
              vous jouez en dernier.
            </TextStandard>
          </Card>
        </Box>

        <Box
          pr={"2"}
          grow={"1"}
          pb={"2"}
          style={{
            width: "50%",
          }}
        >
          <Card>
            <TitleMin title={"Rhinite chronique"} subtitle={"Trait"} />
            <TextStandard>
              Vous éternuez souvent. Lors d'un test de discrétion, vous avez un
              désavantage mineur.
            </TextStandard>
          </Card>
        </Box>

        <Box
          pr={"2"}
          grow={"1"}
          pb={"2"}
          style={{
            width: "50%",
          }}
        >
          <Card>
            <TitleMin
              title={"Réseau vivant"}
              subtitle={"Echo de technomancie"}
            />
            <TextStandard>
              Vous pouvez utiliser votre personna incarné comme noeud PAN.
            </TextStandard>
          </Card>
        </Box>
      </FlexList>
    </Section>
  );
};
