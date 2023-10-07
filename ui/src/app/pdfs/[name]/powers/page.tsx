import { MasonryGrid } from "@/components/MasonryGrid";
import { Space } from "@/components/Space";
import { TitleSection } from "@/components/TitleSection";
import { SimpleAction } from "@/components/actions/SimpleAction";
import { Sprite } from "@/components/items/Sprite";
import { Box } from "@radix-ui/themes";
import { characters } from "resources";
import { PdfContainer } from "../../PdfContainer";
import { SpellAction } from "@/components/actions/SpellAction";
import { ReactNode } from "react";
import { RitualAction } from "@/components/actions/RitualAction";
import { Spirit } from "@/components/items/Spirit";
import { OtherCompanion } from "@/components/items/OtherCompanion";

type Props = {
  params: {
    name: string;
  };
};

export default function Home({ params: { name } }: Props) {
  const char = characters[name];

  return (
    <PdfContainer breakAfter>
      <Box pt={"2"}>
        <MasonryGrid columns={3}>
          <Box>
            <TitleSection>Pouvoirs et compagnons</TitleSection>
            <Space />
          </Box>
          {Object.entries(char.sprites || {}).map(([name, sprite]) => {
            return (
              <Container key={name}>
                <Sprite name={name} sprite={sprite} key={name} />
              </Container>
            );
          })}
          {Object.entries(char.spirits || {}).map(([name, spirit]) => {
            return (
              <Container key={name}>
                <Spirit name={name} spirit={spirit} key={name} />
              </Container>
            );
          })}
          {Object.entries(char.other_companions || {}).map(
            ([name, companion]) => {
              return (
                <Container key={name}>
                  <OtherCompanion
                    name={name}
                    otherCompanion={companion}
                    key={name}
                  />
                </Container>
              );
            }
          )}

          {Object.entries(char.complex_forms || {}).map(([name, form]) => {
            return (
              <Container key={name}>
                <SimpleAction name={name} action={form} type={form.type} />
              </Container>
            );
          })}
          {Object.entries(char.spells || {}).map(([name, spell]) => {
            return (
              <Container key={name}>
                <SpellAction name={name} action={spell} />
              </Container>
            );
          })}
          {Object.entries(char.rituals || {}).map(([name, ritual]) => {
            return (
              <Container key={name}>
                <RitualAction name={name} action={ritual} />
              </Container>
            );
          })}
        </MasonryGrid>
      </Box>
    </PdfContainer>
  );
}

const Container = ({ children }: { children: ReactNode }) => {
  return <Box pb={"2"}>{children}</Box>;
};
