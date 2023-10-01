import { MasonryGrid } from "@/components/MasonryGrid";
import { Section } from "@/components/Section";
import { TitleSection } from "@/components/TitleSection";
import { Effect } from "@/components/actions/Effect";
import { Character } from "resources";

type EffectsProps = {
  char: Character;
};

export const Effects = ({ char }: EffectsProps) => {
  return (
    <Section>
      <MasonryGrid compact columns={4}>
        <TitleSection>Effects</TitleSection>
        {char.effects?.map((e, i) => (
          <Effect key={i} effect={e} />
        ))}
      </MasonryGrid>
    </Section>
  );
};
