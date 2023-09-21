import { Card } from "@/components/Card";
import { FlexList } from "@/components/FlexList";
import { Section } from "@/components/Section";
import { capitalize } from "@/utils/capitalize";
import { Box, Flex, Text } from "@radix-ui/themes";
import { Character } from "resources";

type Skill = Character["skills"][keyof Character["skills"]];

type SkillsProps = {
  char: Character;
};

export const Skills = ({ char }: SkillsProps) => {
  let skills: [string, Skill][] = Object.entries(char.skills) as any;
  return (
    <Section title={"Compétences"}>
      <FlexList>
        {skills.map(([name, value]) => (
          <Container>
            {value && (
              <Card>
                <SkillText name={capitalize(name)} score={value.base} />
                {value.specialisations?.map((name) => (
                  <MasterText text={`${name} +2`} />
                ))}
                {value.expertises?.map((name) => (
                  <MasterText text={`${name} +3`} />
                ))}
              </Card>
            )}
          </Container>
        ))}
      </FlexList>
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
    <Flex justify={"between"}>
      <Text
        weight={"bold"}
        size={"2"}
        style={{
          lineHeight: 1.5,
        }}
      >
        {name}
      </Text>
      <Box pl={"2"} asChild>
        <Text
          weight={"bold"}
          size={"2"}
          style={{
            lineHeight: 1.5,
          }}
        >
          {score}
        </Text>
      </Box>
    </Flex>
  );
};

const MasterText = ({ text }: { text: string }) => {
  return (
    <Flex justify={"between"}>
      <Text
        weight={"light"}
        size={"1"}
        style={{
          lineHeight: 1,
        }}
      >
        <Box pl={"2"}>{text}</Box>
      </Text>
    </Flex>
  );
};
