import { Heading, Text } from "@radix-ui/themes";

type MinTitleProps = {
  title?: React.ReactNode;
  subtitle?: React.ReactNode;
  inline?: boolean;
};

export const TitleMin = ({
  title,
  subtitle,
  inline = false,
}: MinTitleProps) => {
  return (
    <>
      <Heading
        size={"2"}
        as={"h4"}
        style={{
          display: inline ? "inline" : "block",
          lineHeight: 1,
        }}
      >
        {title}
      </Heading>{" "}
      <Text
        size={"1"}
        as="span"
        weight={"light"}
        style={{
          display: inline ? "inline" : "block",
          paddingLeft: inline ? "var(--space-1)" : 0,
          lineHeight: 1,
        }}
      >
        {subtitle}
      </Text>
    </>
  );
};
