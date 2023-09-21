import { Card } from "@/components/Card";
import { FlexList } from "@/components/FlexList";
import { Section } from "@/components/Section";
import { Space } from "@/components/Space";
import { TextStandard } from "@/components/TextStandard";
import { TitleMin } from "@/components/TitleMin";
import { Box } from "@radix-ui/themes";
import { Character } from "resources";

type EffectsProps = {
  char: Character;
};

export const Effects = ({ char }: EffectsProps) => {
  return (
    <Section title="Effets" separator="left">
      <FlexList>
        {char.effects?.map((e) => (
          <Container width="50%">
            <Card title={e.type}>
              <TitleMin title={e.name} />
              <Space />
              <TextStandard>{e.description}</TextStandard>
            </Card>
          </Container>
        ))}
      </FlexList>
    </Section>
  );
};

const Container = ({
  children,
  width,
}: {
  children?: React.ReactNode;
  width?: string;
}) => {
  return (
    <Box
      pr={"2"}
      grow={"1"}
      pb={"2"}
      style={{
        width,
      }}
    >
      {children}
    </Box>
  );
};
