import 'package:flutter/material.dart';

class TagInputWidget extends StatefulWidget {
  TagInputWidget({Key key, this.tags}) : super(key: key);

  // This widget is the home page of your application. It is stateful, meaning
  // that it has a State object (defined below) that contains fields that affect
  // how it looks.

  // This class is the configuration for the state. It holds the values (in this
  // case the title) provided by the parent (in this case the App widget) and
  // used by the build method of the State. Fields in a Widget subclass are
  // always marked "final".

  final List<String> tags;

  @override
  _TagInputState createState() => _TagInputState();
}

class _TagInputState extends State<TagInputWidget> {
  Map<String, bool> _selectedTags;

  @override
  void initState() {
    super.initState();
    _selectedTags = new Map<String, bool>();
    for (String tag in widget.tags) {
      _selectedTags[tag] = false;
    }
  }

  void addTag(String tag) {
    setState(() {
      _selectedTags[tag] = true;
    });
  }

  @override
  Widget build(BuildContext context) {
    List<Widget> chips = <Widget>[];
    for (String tag in _selectedTags.keys) {
      Widget chip = InputChip(
        label: Text(tag),
        selected: _selectedTags[tag],
        onSelected: (bool newValue) {
          setState(() {
            _selectedTags[tag] = newValue;
          });
        },
      );
      chips.add(chip);
    }
    return SingleChildScrollView(
        scrollDirection: Axis.horizontal,
        child: Row(
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: chips,
        ));
  }
}
