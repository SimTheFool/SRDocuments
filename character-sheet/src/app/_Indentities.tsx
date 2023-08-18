import { Card } from "@/components/Card";
import { FlexList } from "@/components/FlexList";
import { Section } from "@/components/Section";
import { TextStandard } from "@/components/TextStandard";
import { TitleMin } from "@/components/TitleMin";
import { Box } from "@radix-ui/themes";

type IdentitiesProps = {};

export const Identities = ({}: IdentitiesProps) => {
  return (
    <Section title="Identités" separator="left">
      <FlexList>
        <Box
          pr={"2"}
          grow={"1"}
          pb={"2"}
          style={{
            minWidth: "95%",
          }}
        >
          <Card title={"Détails"}>
            <TitleMin title={"Laurence Guinvite"} subtitle={"SIN I4 - 5000Y"} />
            <TextStandard>1m80, 80kg, 18 ans. Coursier</TextStandard>
          </Card>
        </Box>
        <Box
          pr={"2"}
          grow={"1"}
          pb={"2"}
          style={{
            minWidth: "50%",
          }}
        >
          <Card title={"Nuyens"}>
            <TextStandard>_</TextStandard>
          </Card>
        </Box>
        <Box
          pr={"2"}
          grow={"1"}
          pb={"2"}
          style={{
            minWidth: "50%",
          }}
        >
          <Card title={"Style de vie"}>
            <TextStandard>Squatteur</TextStandard>
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
          <Card title={"Contact"}>
            <TitleMin title={"D-Boss"} subtitle={"L4 - R4"} />
            <TextStandard>Decker fan de complot</TextStandard>
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
          <Card title={"Contact"}>
            <TitleMin title={"Terrance"} subtitle={"L3 - R2"} />
            <TextStandard>Ouvrier de casse militaire d'ARES</TextStandard>
          </Card>
        </Box>
      </FlexList>
    </Section>
  );
};
