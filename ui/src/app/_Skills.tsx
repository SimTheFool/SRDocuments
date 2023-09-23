import { Card } from "@/components/Card";
import { FlexList } from "@/components/FlexList";
import { Section } from "@/components/Section";
import { Space } from "@/components/Space";
import { TitleSection } from "@/components/TitleSection";
import { capitalize } from "@/utils/capitalize";
import { uncapitalize } from "@/utils/uncapitalize";
import { Box, Flex, Text } from "@radix-ui/themes";
import { Character } from "resources";

type Skill = Character["skills"][keyof Character["skills"]];

type SkillsProps = {
  char: Character;
};

export const Skills = ({ char }: SkillsProps) => {
  let skills: [string, Skill][] = Object.entries(char.skills) as any;
  return (
    <Section title={<TitleSection>Compétences</TitleSection>}>
      <Box
        style={{
          display: "column",
          columnCount: 2,
          columnGap: "0",
        }}
      >
        {skills.map(([name, value]) => (
          <Container>
            {value && (
              <Card>
                <SkillText name={capitalize(name)} score={value.base} />
                {value.specialisations?.map((name) => (
                  <MasterText score={2} label={name} />
                ))}
                {value.expertises?.map((name) => (
                  <MasterText score={3} label={name} />
                ))}
              </Card>
            )}
          </Container>
        ))}
      </Box>
    </Section>
  );
};

const Container = ({ children }: { children: React.ReactNode }) => {
  return (
    <Box pr={"2"} pb={"2"} grow={"1"}>
      {children}
    </Box>
  );
};

const SkillText = ({ name, score }: { name: string; score: number }) => {
  return (
    <Flex
      justify={"between"}
      style={{
        width: "100%",
      }}
    >
      <Text
        weight={"bold"}
        size={"2"}
        style={{
          maxWidth: "90%",
          flexShrink: 1,
          lineHeight: 1.5,
          whiteSpace: "nowrap",
          overflow: "hidden",
          textOverflow: "ellipsis",
        }}
      >
        {name}
      </Text>
      <Box pl={"1"} asChild>
        <Text
          weight={"bold"}
          size={"2"}
          style={{
            flexShrink: 0,
            lineHeight: 1.5,
          }}
        >
          {score}
        </Text>
      </Box>
    </Flex>
  );
};

const MasterText = ({ label, score }: { label: string; score: number }) => {
  return (
    <Flex pl={"2"}>
      <Box pr={"1"} asChild>
        <Text
          weight={"light"}
          size={"1"}
          style={{
            lineHeight: 1,
          }}
        >
          +{score}
        </Text>
      </Box>
      <Text
        weight={"light"}
        size={"1"}
        style={{
          maxWidth: "90%",
          flexShrink: 1,
          lineHeight: 1,
          whiteSpace: "nowrap",
          overflow: "hidden",
          textOverflow: "ellipsis",
        }}
      >
        {uncapitalize(label)}
      </Text>
    </Flex>
  );
};
