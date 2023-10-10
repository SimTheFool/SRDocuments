import { MasonryGrid } from "@/components/MasonryGrid";
import { Section } from "@/components/Section";
import { TitleSection } from "@/components/TitleSection";
import { Effect } from "@/components/actions/Effect";
import { Box } from "@radix-ui/themes";
import { Character } from "resources";

type EffectsProps = {
  char: Character;
};

export const Effects = ({ char }: EffectsProps) => {
  return (
    <Section>
      <MasonryGrid compact columns={4}>
        <TitleSection>Effets</TitleSection>
        {char.effects?.map((e, i) => (
          <Box key={i} pr={"2"} pb={"2"}>
            <Effect effect={e} />
          </Box>
        ))}
      </MasonryGrid>
    </Section>
  );
};
