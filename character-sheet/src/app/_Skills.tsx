import { Card } from "@/components/Card";
import { FlexList } from "@/components/FlexList";
import { Section } from "@/components/Section";
import { Box, Flex, Text } from "@radix-ui/themes";

type Skill = {
  name: string;
  score: number;
  master: {
    name: string;
    bonus: number;
  }[];
};

type SkillsProps = {
  skills: Skill[];
};

export const Skills = ({ skills }: SkillsProps) => {
  return (
    <Section title={"Compétences"}>
      <FlexList>
        {skills.map(({ name, score, master }) => (
          <Container>
            <Card>
              <SkillText name={name} score={score} />
              {master.map(({ name, bonus }) => (
                <MasterText text={`${name} +${bonus}`} />
              ))}
            </Card>
          </Container>
        ))}
      </FlexList>
    </Section>
  );
};

const Container = ({ children }: { children: React.ReactNode }) => {
  return (
    <Box
      pr={"2"}
      grow={"1"}
      pb={"2"}
      style={{
        width: "130px",
      }}
    >
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
      <Text
        weight={"bold"}
        size={"2"}
        style={{
          lineHeight: 1.5,
        }}
      >
        {score}
      </Text>
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
