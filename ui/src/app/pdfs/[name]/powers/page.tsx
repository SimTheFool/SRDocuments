import { MasonryGrid } from "@/components/MasonryGrid";
import { Space } from "@/components/Space";
import { TitleSection } from "@/components/TitleSection";
import { SimpleAction } from "@/components/actions/SimpleAction";
import { Sprite } from "@/components/items/Sprite";
import { Box } from "@radix-ui/themes";
import { Character, characters } from "resources";
import { PdfBreak, PdfContainer } from "../../PdfContainer";
import { SpellAction } from "@/components/actions/SpellAction";
import { ReactNode } from "react";
import { RitualAction } from "@/components/actions/RitualAction";
import { Spirit } from "@/components/items/Spirit";
import { OtherCompanion } from "@/components/items/OtherCompanion";
import { getCharWeights } from "@/utils/getWeights";
import { Line, LineBlack } from "@/components/Line";

type Props = {
  params: {
    name: string;
  };
};

export default function Home({ params: { name } }: Props) {
  const char = characters[name];
  const charWeight = getCharWeights(char);
  const pageWeight = charWeight.powers + charWeight.companions;

  if (pageWeight < 20)
    return (
      <PdfContainer footer={"POUVOIRS"}>
        <AllInOne char={char} />
      </PdfContainer>
    );

  return (
    <>
      <PdfContainer footer={"POUVOIRS"}>
        <ActionOnly char={char} />
      </PdfContainer>
      <PdfBreak />
      <PdfContainer footer={"COMPAGNONS"}>
        <CompanionOnly char={char} />
      </PdfContainer>
    </>
  );
}

const ActionOnly = ({ char }: { char: Character }) => {
  return (
    <MasonryGrid columns={3}>
      <Box>
        <TitleSection>Pouvoirs</TitleSection>
        <Space />
      </Box>

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
  );
};

const CompanionOnly = ({ char }: { char: Character }) => {
  return (
    <MasonryGrid columns={1}>
      <Box>
        <TitleSection>Compagnons</TitleSection>
        <Space />
      </Box>

      {Object.entries(char.sprites || {}).map(([name, sprite]) => {
        return (
          <Container key={name}>
            <Sprite name={name} sprite={sprite} key={name} ergo />
          </Container>
        );
      })}

      <LineBlack />

      {Object.entries(char.spirits || {}).map(([name, spirit]) => {
        return (
          <Container key={name}>
            <Spirit name={name} spirit={spirit} key={name} ergo />
          </Container>
        );
      })}

      <LineBlack />

      {Object.entries(char.other_companions || {}).map(([name, companion]) => {
        return (
          <Container key={name}>
            <OtherCompanion
              name={name}
              otherCompanion={companion}
              key={name}
              ergo
            />
          </Container>
        );
      })}
    </MasonryGrid>
  );
};

const AllInOne = ({ char }: { char: Character }) => {
  return (
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
      {Object.entries(char.other_companions || {}).map(([name, companion]) => {
        return (
          <Container key={name}>
            <OtherCompanion name={name} otherCompanion={companion} key={name} />
          </Container>
        );
      })}

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
  );
};

const Container = ({ children }: { children: ReactNode }) => {
  return (
    <Box pb={"2"} pr={"2"}>
      {children}
    </Box>
  );
};
