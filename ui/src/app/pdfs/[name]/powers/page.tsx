import { MasonryGrid } from "@/components/MasonryGrid";
import { Space } from "@/components/Space";
import { TitleSection } from "@/components/TitleSection";
import { BaseAction } from "@/components/actions/BaseAction";
import { Sprite } from "@/components/items/Sprite";
import { Box } from "@radix-ui/themes";
import { characters } from "resources";
import { PdfContainer } from "../../PdfContainer";
import { SpellAction } from "@/components/actions/SpellAction";
import { ReactNode } from "react";

type Props = {
  params: {
    name: string;
  };
};

export default function Home({ params: { name } }: Props) {
  const char = characters[name];

  return (
    <PdfContainer>
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
          {Object.entries(char.actions || {}).map(([name, action]) => {
            return (
              <Container key={name}>
                <BaseAction name={name} action={action} type={action.type} />
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
        </MasonryGrid>
      </Box>
    </PdfContainer>
  );
}

const Container = ({ children }: { children: ReactNode }) => {
  return <Box pb={"2"}>{children}</Box>;
};
