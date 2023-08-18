import { Heading, Text } from "@radix-ui/themes";

type MinTitleProps = {
  title?: React.ReactNode;
  subtitle?: React.ReactNode;
};

export const TitleMin = ({ title, subtitle }: MinTitleProps) => {
  return (
    <>
      <Heading
        size={"2"}
        as={"h4"}
        style={{
          lineHeight: 1,
        }}
      >
        {title}
      </Heading>
      <Text
        size={"1"}
        as="span"
        weight={"light"}
        style={{
          display: "block",
          lineHeight: 1,
          paddingBottom: "var(--space-1)",
        }}
      >
        {subtitle}
      </Text>
    </>
  );
};
